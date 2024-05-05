use std::{io::Error, net::TcpListener, sync::Arc, thread};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::server::{
    prelude::{Listener, DEVICES},
    tcp::handle_client,
};

use super::prelude::Certificate;

///
/// Listens for incoming TLS connections.
///
/// It creates a new thread for SHDP clients.
///
/// # Arguments
/// * `port` - The port to listen on.
/// * `cert` - The certificate to use.
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
/// use shdp::prelude::server::tls::listen;
///
/// let cert = shdp::prelude::server::Certificate {
///     cert_path: String::from("cert.pem"),
///     key_path: String::from("key.pem"),
/// };
///
/// match listen(String::from("8080"), cert) {
///     Ok(_) => println!("Listening on port 8080"),
///     Err(e) => println!("Error: {:?}", e),
/// }
/// ```
pub fn listen(port: String, cert: Certificate) -> Result<(), Error> {
    let acceptor = load_acceptor(cert);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    DEVICES.lock().unwrap().insert(
        ("127.0.0.1".to_string(), port.clone()),
        Listener::Sync(listener),
    );

    println!("[SHDP:TLS] Listening on port {}", port.clone());

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
