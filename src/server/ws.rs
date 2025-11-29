use std::sync::Arc;

use async_tungstenite::{accept_async, WebSocketStream};
use bitvec::order::{Lsb0, Msb0};
use futures::{AsyncRead, AsyncWrite};
use smol::{net::TcpListener, stream::StreamExt};
use tokio::sync::Mutex;
use tungstenite::Message;

use crate::protocol::{
    errors::{Error, ErrorKind},
    managers::bits::{
        decoder::{BitDecoder, FrameDecoder},
        encoder::FrameEncoder,
    },
    prelude::common::{
        registry::EVENT_REGISTRY_MSB,
        utils::{Listener, DEVICES},
    },
    versions::Version,
};

use super::prelude::answer_error;

///
/// Listens for incoming WebSocket connections.
///
/// It creates a new thread for SHDP clients.
///
/// # Arguments
/// * `port` - The port to listen on.
///
/// # Returns
/// * [Result<(), ShdpError>] - The result of the operation.
///
/// # Errors
/// Generated errors are related to the I/O operations.<br>
/// They need to be handled by the caller.
///
/// # Example
/// ```rust,no_run
/// use shdp::prelude::server::ws::listen;
///
/// #[tokio::main]
/// async fn main() {
///     match listen(String::from("8080")).await {
///         Ok(_) => println!("Listening on port 8080"),
///         Err(e) => println!("Error: {:?}", e),
///     }
/// }
/// ```
pub async fn listen(port: String) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    let static_listener: &'static TcpListener = Box::leak(Box::new(listener));

    {
        let mut devices = DEVICES.lock().map_err(|_| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Mutex poisoned",
            )) as Box<dyn std::error::Error>
        })?;
        devices.insert(
            ("127.0.0.1".to_string(), port.clone()),
            Listener::StdServer(static_listener.clone()),
        );
    }

    println!("[SHDP:WS] Listening on port {}", port.clone());

    loop {
        let (stream, _) = match static_listener.accept().await {
            Ok(stream) => stream,
            Err(e) => {
                println!("[SHDP:WS] Error accepting connection: {}", e);
                continue;
            }
        };
        let ws = accept_async(stream.clone()).await;

        match ws {
            Ok(ws_stream) => {
                let executor = smol::LocalExecutor::new();
                executor
                    .spawn(async move {
                        handle_connection(Arc::new(Mutex::new(ws_stream))).await;
                    })
                    .detach();
            }
            Err(e) => {
                println!("[SHDP:WS] Error accepting WebSocket connection: {}", e);
            }
        }

        println!("[SHDP:WS] New connection from {}", stream.peer_addr()?);
    }
}

pub async fn handle_connection<IO: AsyncRead + AsyncWrite + Unpin>(
    ws: Arc<Mutex<WebSocketStream<IO>>>,
) {
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
            if let Err(e) = guard.send(Message::Binary(err.into())).await {
                println!("[SHDP:WS] Error sending error message: {}", e);
            }

            break;
        }

        handle_message(Arc::clone(&ws), message).await;
    }
}

async fn handle_message<IO: AsyncRead + AsyncWrite + Unpin>(
    ws: Arc<Mutex<WebSocketStream<IO>>>,
    message: Message,
) {
    let data = message.into_data();
    let decoder = BitDecoder::<Msb0>::new(data.into());
    let data = FrameDecoder::<Msb0>::new(decoder.clone()).decode().unwrap();

    let factory = {
        let registry = EVENT_REGISTRY_MSB.lock().unwrap();
        match registry.get_event((data.version, data.event)) {
            Some(event) => *event,
            None => {
                drop(registry);
                let err = answer_error(
                    data.version,
                    Error {
                        code: 404,
                        message: "Event not found".to_string(),
                        kind: ErrorKind::NotFound,
                    },
                );

                let mut guard = ws.lock().await;
                if let Err(e) = guard.send(Message::Binary(err.into())).await {
                    println!("[SHDP:WS] Error sending error message: {}", e);
                }

                return;
            }
        }
    };

    let mut event = factory(decoder);
    match event.decode(data.clone()) {
        Ok(_) => (),
        Err(e) => {
            let err = answer_error(data.version, e);

            let mut guard = ws.lock().await;
            if let Err(e) = guard.send(Message::Binary(err.into())).await {
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
            if let Err(e) = guard.send(Message::Binary(err.into())).await {
                println!("[SHDP:WS] Error sending error message: {}", e);
            }

            return;
        }
    };

    for response in responses {
        let mut encoder = match FrameEncoder::<Lsb0>::new(data.version) {
            Ok(encoder) => encoder,
            Err(e) => {
                println!("[SHDP:WS] Error creating encoder: {}", e);
                return;
            }
        };

        let frame = encoder.encode(response).unwrap();

        let mut guard = ws.lock().await;
        if let Err(e) = guard.send(Message::Binary(frame.into())).await {
            println!("[SHDP:WS] Error sending response: {}", e);
            return;
        }
    }
}
