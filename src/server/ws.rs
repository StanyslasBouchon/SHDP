use std::sync::Arc;

use async_std::{
    net::{TcpListener, TcpStream},
    stream::StreamExt,
};
use async_tungstenite::{accept_async, WebSocketStream};
use futures::SinkExt;
use tokio::sync::Mutex;
use tungstenite::Message;

use crate::protocol::{
    errors::{Error, ErrorKind},
    server::{
        bits::decoder::InDecoder,
        builder::OutBuilder,
        decoder::Decoder,
        versions::{registry::EVENT_REGISTRY, v1::c0x0002::ErrorResponse},
    },
    versions::Version,
};

pub async fn listen(port: String) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    println!("[SHDP:WS] Listening on port {}", port);

    while let Ok((stream, _)) = listener.accept().await {
        let ws = accept_async(stream.clone()).await;

        match ws {
            Ok(ws_stream) => {
                async_std::task::spawn_local(async move {
                    handle_connection(Arc::new(Mutex::new(ws_stream))).await;
                });
            }
            Err(e) => {
                println!("[SHDP:WS] Error accepting WebSocket connection: {}", e);
            }
        }

        println!("[SHDP:WS] New connection from {}", stream.peer_addr()?);
    }

    Ok(())
}

async fn handle_connection(ws: Arc<Mutex<WebSocketStream<TcpStream>>>) {
    while let Some(message) = {
        let mut guard = ws.lock().await;
        guard.next().await
    } {
        if message.is_err() {
            println!("[SHDP:WS] Error reading from WebSocket: {:?}", message);
            break;
        }

        let message = message.unwrap();

        if !message.is_binary() {
            let err = answer_error(
                Version::V1.to_u8(),
                Error {
                    code: 400,
                    message: "Bad Request".to_string(),
                    kind: ErrorKind::BadRequest,
                },
            );

            let mut guard = ws.lock().await;
            if let Err(e) = guard.send(Message::Binary(err)).await {
                println!("[SHDP:WS] Error sending error message: {}", e);
            }

            break;
        }

        handle_message(Arc::clone(&ws), message).await;
    }
}

async fn handle_message(ws: Arc<Mutex<WebSocketStream<TcpStream>>>, message: Message) {
    let data = message.into_data();
    let decoder = InDecoder::new(data);
    let data = Decoder::new(decoder.clone()).parse().unwrap();

    let factory = match EVENT_REGISTRY.get_event(data.version, data.event) {
        Some(event) => event,
        None => {
            println!(
                "[SHDP:TCP] Event not found: {} <-> {}",
                data.version, data.event
            );

            let err = answer_error(
                data.version,
                Error {
                    code: 404,
                    message: "Event not found".to_string(),
                    kind: ErrorKind::NotFound,
                },
            );

            let mut guard = ws.lock().await;
            if let Err(e) = guard.send(Message::Binary(err)).await {
                println!("[SHDP:WS] Error sending error message: {}", e);
            }

            return;
        }
    };

    let mut event = factory(decoder);
    match event.parse() {
        Ok(_) => (),
        Err(e) => {
            let err = answer_error(data.version, e);

            let mut guard = ws.lock().await;
            if let Err(e) = guard.send(Message::Binary(err)).await {
                println!("[SHDP:WS] Error sending error message: {}", e);
            }

            return;
        }
    }

    let responses = match event.get_responses() {
        Ok(responses) => responses,
        Err(e) => {
            let err = answer_error(data.version, e);

            let mut guard = ws.lock().await;
            if let Err(e) = guard.send(Message::Binary(err)).await {
                println!("[SHDP:WS] Error sending error message: {}", e);
            }

            return;
        }
    };

    for response in responses {
        let mut builder = match OutBuilder::new(data.version) {
            Ok(builder) => builder,
            Err(e) => {
                println!("[SHDP:WS] Error creating builder: {}", e);
                return;
            }
        };

        let frame = builder.construct(response).unwrap();

        let mut guard = ws.lock().await;
        if let Err(e) = guard.send(Message::Binary(frame)).await {
            println!("[SHDP:WS] Error sending response: {}", e);
            return;
        }
    }
}

fn answer_error(version: u8, error: crate::protocol::errors::Error) -> Vec<u8> {
    let mut builder = OutBuilder::new(version).unwrap();
    builder
        .construct(Box::new(ErrorResponse::new(error)))
        .unwrap()
}
