use std::{io::Error, net::TcpListener, sync::Arc, thread};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::server::tcp::handle_client;

pub struct Certificate {
    pub cert_path: String,
    pub key_path: String,
}

pub fn listen(port: String, cert: Certificate) -> Result<(), Error> {
    let acceptor = load_acceptor(cert);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    println!("[SHDP:TLS] Listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!(
                    "[SHDP:TLS] New connection from {}",
                    stream.peer_addr().unwrap()
                );

                let acceptor_duplicate = acceptor.clone();
                thread::spawn(move || {
                    let tls_stream = acceptor_duplicate.accept(stream);

                    match tls_stream {
                        Ok(tls_stream) => {
                            handle_client(tls_stream);
                        }
                        Err(e) => println!("[SHDP:TLS] Error: {:?}", e),
                    }
                });
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn load_acceptor(cert: Certificate) -> Arc<SslAcceptor> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&cert.key_path, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(&cert.cert_path).unwrap();
    Arc::new(builder.build())
}
