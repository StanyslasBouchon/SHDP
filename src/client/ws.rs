use std::io::{Read, Write};
use std::sync::Arc;

use bitvec::order::{Lsb0, Msb0};
use std::net::TcpStream;
use tokio::sync::Mutex;
use tungstenite::{client, WebSocket};
use tungstenite::{client::IntoClientRequest, Message};

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
};

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
/// use shdp::prelude::client::ws::connect;
///
/// #[tokio::main]
/// async fn main() {
///     match connect(String::from("157.165.164.160"), String::from("8080")).await {
///         Ok(_) => println!("Connection established"),
///         Err(e) => println!("Error: {:?}", e),
///     }
/// }
/// ```
pub async fn connect(ip: String, port: String) -> Result<(), Error> {
    let stream = match TcpStream::connect(format!("{}:{}", ip, port)) {
        Ok(stream) => stream,
        Err(e) => {
            println!(
                "[SHDP:TCP] Error connecting to {}:{}",
                ip.clone(),
                port.clone()
            );
            return Err(Error {
                code: 0,
                message: e.to_string(),
                kind: ErrorKind::UserDefined(Box::new(e)),
            });
        }
    };
    let static_stream: &'static mut TcpStream = Box::leak(Box::new(stream));

    DEVICES.lock().unwrap().insert(
        (ip.clone(), port.clone()),
        Listener::StdClient(static_stream),
    );

    println!("[SHDP:TCP] Connected to {}:{}", ip.clone(), port.clone());

    let mut devices = DEVICES.lock().unwrap();
    let real_stream = devices
        .get_mut(&(ip.clone(), port.clone()))
        .unwrap()
        .get_std_client();

    let (ws_stream, _) = client(
        match format!("ws://{}:{}", ip.clone(), port.clone()).into_client_request() {
            Ok(request) => request,
            Err(e) => {
                println!("[SHDP:WS] Error creating request: {}", e);
                return Err(Error {
                    code: 0,
                    message: e.to_string(),
                    kind: ErrorKind::UserDefined(Box::new(e)),
                });
            }
        },
        real_stream,
    )
    .unwrap();

    let _ = handle_connection(Arc::new(Mutex::new(ws_stream))).await;

    Ok(())
}

pub async fn handle_connection<IO: Read + Write + Unpin>(
    ws: Arc<Mutex<WebSocket<IO>>>,
) -> Result<(), Error> {
    while let Some(message) = {
        let mut guard = ws.lock().await;
        Some(guard.read().unwrap())
    } {
        if !message.is_binary() {
            return Err(Error {
                code: 400,
                message: "Bad Request".to_string(),
                kind: ErrorKind::BadRequest,
            });
        }

        let _ = handle_message(Arc::clone(&ws), message).await;
    }

    Ok(())
}

async fn handle_message<IO: Read + Write + Unpin>(
    ws: Arc<Mutex<WebSocket<IO>>>,
    message: Message,
) -> Result<(), Error> {
    let data = message.into_data();
    let decoder = BitDecoder::<Msb0>::new(data);
    let data = FrameDecoder::<Msb0>::new(decoder.clone()).decode().unwrap();

    let registry = EVENT_REGISTRY_MSB.lock().unwrap();
    let factory = match registry.get_event((data.version, data.event)) {
        Some(event) => event,
        None => {
            println!(
                "[SHDP:TCP] Event not found: {} <-> {}",
                data.version, data.event
            );

            return Err(Error {
                code: 404,
                message: "Event not found".to_string(),
                kind: ErrorKind::NotFound,
            });
        }
    };

    let mut event = factory(decoder);
    event.decode(data.clone())?;

    let responses = event.get_responses()?;

    for response in responses {
        let mut encoder = match FrameEncoder::<Lsb0>::new(data.version) {
            Ok(encoder) => encoder,
            Err(e) => {
                println!("[SHDP:WS] Error creating encoder: {}", e);
                return Err(Error {
                    code: 0,
                    message: e.to_string(),
                    kind: ErrorKind::UserDefined(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Unknown",
                    ))),
                });
            }
        };

        let frame = encoder.encode(response).unwrap();

        let mut guard = ws.lock().await;
        if let Err(e) = guard.send(Message::Binary(frame)) {
            println!("[SHDP:WS] Error sending response: {}", e);
            return Err(Error {
                code: 0,
                message: e.to_string(),
                kind: ErrorKind::UserDefined(Box::new(e)),
            });
        }
    }

    Ok(())
}
