use std::{
    io::{Error, Read, Write},
    net::TcpListener,
    thread,
};

use crate::protocol::server::{
    bits::decoder::InDecoder,
    builder::OutBuilder,
    decoder::Decoder,
    versions::{registry::EVENT_REGISTRY, v1::c0x0002::ErrorResponse},
};

pub fn listen(port: String) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    println!("[SHDP:TCP] Listening on port {}", port);

    for stream in listener.incoming() {
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
                let decoder = InDecoder::new(buffer[..size].to_vec());
                let data = Decoder::new(decoder.clone()).parse().unwrap();

                let factory = match EVENT_REGISTRY.get_event(data.version, data.event) {
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
                match event.parse() {
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
                    let mut builder = match OutBuilder::new(data.version) {
                        Ok(builder) => builder,
                        Err(e) => {
                            println!("[SHDP:TCP] Error creating builder: {}", e);
                            return;
                        }
                    };

                    let frame = builder.construct(response).unwrap();

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

fn answer_error(version: u8, error: crate::protocol::errors::Error) -> Vec<u8> {
    let mut builder = OutBuilder::new(version).unwrap();
    builder
        .construct(Box::new(ErrorResponse::new(error)))
        .unwrap()
}
