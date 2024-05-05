use std::{
    io::{Error, Read, Write},
    net::TcpListener,
    thread,
};

use bitvec::order::{Lsb0, Msb0};

use crate::{
    protocol::{
        managers::bits::{
            decoder::{BitDecoder, FrameDecoder},
            encoder::FrameEncoder,
        },
        prelude::common::registry::EVENT_REGISTRY_MSB,
    },
    server::prelude::{answer_error, Listener, DEVICES},
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
/// use shdp::prelude::server::tcp::listen;
///
/// match listen(String::from("8080")) {
///     Ok(_) => println!("Listening on port 8080"),
///     Err(e) => println!("Error: {:?}", e),
/// }
/// ```
pub fn listen(port: String) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    DEVICES.lock().unwrap().insert(
        ("127.0.0.1".to_string(), port.clone()),
        Listener::Sync(listener),
    );

    println!("[SHDP:TCP] Listening on port {}", port.clone());

    for stream in DEVICES
        .lock()
        .unwrap()
        .get(&("127.0.0.1".to_string(), port.clone()))
        .unwrap()
        .get_sync()
        .incoming()
    {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    println!(
                        "[SHDP:TCP] New connection from {}",
                        stream.peer_addr().unwrap()
                    );

                    handle_client(stream);
                });
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

pub fn handle_client<S: Read + Write>(mut stream: S) {
    let mut buffer = [0u8; 2usize.pow(32) / 8];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(size) => {
                let decoder = BitDecoder::<Msb0>::new(buffer[..size].to_vec());
                let data = FrameDecoder::<Msb0>::new(decoder.clone()).decode().unwrap();

                let registry = EVENT_REGISTRY_MSB.lock().unwrap();
                let factory = match registry.get_event((data.version, data.event)) {
                    Some(event) => event,
                    None => {
                        println!(
                            "[SHDP:TCP] Event not found: {} <-> {}",
                            data.version, data.event
                        );

                        stream
                            .write_all(&answer_error(
                                data.version,
                                crate::protocol::errors::Error {
                                    code: 404,
                                    message: "Event not found".to_string(),
                                    kind: crate::protocol::errors::ErrorKind::NotFound,
                                },
                            ))
                            .unwrap();

                        return;
                    }
                };

                let mut event = factory(decoder);
                match event.decode() {
                    Ok(_) => (),
                    Err(e) => {
                        stream.write_all(&answer_error(data.version, e)).unwrap();
                        return;
                    }
                }

                let responses = match event.get_responses() {
                    Ok(responses) => responses,
                    Err(e) => {
                        stream.write_all(&answer_error(data.version, e)).unwrap();
                        return;
                    }
                };

                for response in responses {
                    let mut encoder = match FrameEncoder::<Lsb0>::new(data.version) {
                        Ok(encoder) => encoder,
                        Err(e) => {
                            println!("[SHDP:TCP] Error creating encoder: {}", e);
                            return;
                        }
                    };

                    let frame = encoder.encode(response).unwrap();

                    match stream.write_all(&frame) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("[SHDP:TCP] Error writing to stream: {}", e);
                            return;
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
}
