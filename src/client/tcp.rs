use bitvec::order::{Lsb0, Msb0};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
};

use crate::protocol::{
    managers::bits::{
        decoder::{BitDecoder, FrameDecoder},
        encoder::FrameEncoder,
    },
    prelude::common::{
        error::{Error, ErrorKind},
        registry::EVENT_REGISTRY_MSB,
        utils::{Listener, DEVICES},
    },
};

///
/// Listens for incoming TCP connections.
///
/// It creates a new thread for SHDP clients.
///
/// # Arguments
/// * `port` - The port to listen on.
///
/// # Returns
/// * [Result<(), Error>] - The result of the operation.
///
/// # Errors
/// Generated errors are related to the I/O operations.<br>
/// They need to be handled by the caller.
///
/// # Example
/// ```rust,no_run
/// use shdp::prelude::client::tcp::connect;
///
/// #[tokio::main]
/// async fn main() {
///     match connect(String::from("157.165.164.160"), String::from("8080")).await {
///         Ok(_) => println!("Connection established"),
///         Err(e) => println!("Error: {:?}", e),
///     }
/// }
///
/// ```
pub async fn connect(ip: String, port: String) -> Result<(), Error> {
    let stream = match TcpStream::connect(format!("{}:{}", ip, port)).await {
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
        Listener::TokioClient(static_stream),
    );

    println!("[SHDP:TCP] Connected to {}:{}", ip.clone(), port.clone());

    let mut devices = DEVICES.lock().unwrap();
    let real_stream = devices
        .get_mut(&(ip.clone(), port.clone()))
        .unwrap()
        .get_tokio_client();

    let _ = handle_client(real_stream).await;

    Ok(())
}

pub async fn handle_client<IO: AsyncRead + AsyncWrite + Unpin>(
    stream: &mut IO,
) -> Result<(), Error> {
    let mut buffer = [0u8; 2usize.pow(32) / 8];

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break,
            Ok(size) => {
                let mut decoder = BitDecoder::<Msb0>::new(buffer[..size].to_vec());
                let mut frame_decoder = FrameDecoder::<Msb0>::new(decoder);
                let data = frame_decoder.decode().unwrap();
                decoder = frame_decoder.get_decoder().to_owned();

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
                            kind: crate::protocol::errors::ErrorKind::NotFound,
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
                            println!("[SHDP:TCP] Error creating encoder: {}", e);
                            return Err(e);
                        }
                    };

                    let frame = encoder.encode(response).unwrap();

                    match stream.write_all(&frame).await {
                        Ok(_) => (),
                        Err(e) => {
                            println!("[SHDP:TCP] Error writing to stream: {}", e);
                            return Err(Error {
                                code: 0,
                                message: e.to_string(),
                                kind: ErrorKind::UserDefined(Box::new(e)),
                            });
                        }
                    }
                }
            }
            Err(e) => {
                println!("[SHDP:TCP] Error reading from stream: {}", e);
                break;
            }
        }
    }

    println!("[SHDP:TCP] Connection closed");

    Ok(())
}
