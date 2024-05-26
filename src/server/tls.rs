use std::{io::Error, sync::Arc, thread};

use openssl::ssl::{Ssl, SslAcceptor, SslFiletype, SslMethod};
use tokio::net::TcpListener;
use tokio_openssl::SslStream;

use crate::{
    protocol::prelude::common::utils::{Certificate, Listener, DEVICES},
    server::tcp::handle_client,
};

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
/// use shdp::prelude::common::utils::Certificate;
///
/// #[tokio::main]
/// async fn main() {
///     match listen(String::from("8080"), Certificate {
///         cert_path: String::from("cert.pem"),
///         key_path: String::from("key.pem"),
///     }).await {
///         Ok(_) => println!("Listening on port 8080"),
///         Err(e) => println!("Error: {:?}", e),
///     }
/// }
/// ```
#[allow(unused_must_use)]
pub async fn listen(port: String, cert: Certificate) -> Result<(), Error> {
    let acceptor = load_acceptor(cert);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    let static_listener: &'static TcpListener = Box::leak(Box::new(listener));

    DEVICES.lock().unwrap().insert(
        ("127.0.0.1".to_string(), port.clone()),
        Listener::TokioServer(static_listener),
    );

    println!("[SHDP:TLS] Listening on port {}", port.clone());

    while let Ok((stream, _)) = DEVICES
        .lock()
        .unwrap()
        .get(&("127.0.0.1".to_string(), port.clone()))
        .unwrap()
        .get_tokio_server()
        .accept()
        .await
    {
        println!(
            "[SHDP:TLS] New connection from {}",
            stream.peer_addr().unwrap()
        );

        let acceptor_duplicate = acceptor.clone();
        thread::spawn(move || {
            let ssl = Ssl::new(acceptor_duplicate.context()).unwrap();
            let tls_stream = SslStream::new(ssl, stream);

            match tls_stream {
                Ok(tls_stream) => {
                    handle_client(tls_stream);
                }
                Err(e) => println!("[SHDP:TLS] Error: {:?}", e),
            }
        });
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
