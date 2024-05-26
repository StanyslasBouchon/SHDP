use std::{net::TcpStream, sync::Arc};

use openssl::ssl::{Ssl, SslConnector, SslFiletype, SslMethod, SslStream};
use tokio::sync::Mutex;
use tungstenite::client;
use tungstenite::client::IntoClientRequest;

use crate::{
    client::ws::handle_connection,
    protocol::{
        errors::{Error, ErrorKind},
        prelude::common::utils::{Certificate, Listener, DEVICES},
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
/// use shdp::prelude::client::wss::connect;
/// use shdp::prelude::common::utils::Certificate;
///
/// #[tokio::main]
/// async fn main() {
///     match connect(String::from("157.165.164.160"), String::from("8080"), Certificate {
///         cert_path: String::from("cert.pem"),
///         key_path: String::from("key.pem"),
///     }).await {
///         Ok(_) => println!("Connection established"),
///         Err(e) => println!("Error: {:?}", e),
///     }
/// }
/// ```
pub async fn connect(ip: String, port: String, cert: Certificate) -> Result<(), Error> {
    let connector = load_connector(cert);
    let stream = match TcpStream::connect(format!("{}:{}", ip, port)) {
        Ok(stream) => stream,
        Err(e) => {
            println!(
                "[SHDP:WSS] Error connecting to {}:{}",
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

    println!("[SHDP:WSS] Connected to {}:{}", ip.clone(), port.clone());

    let mut devices = DEVICES.lock().unwrap();
    let real_stream = devices
        .get_mut(&(ip.clone(), port.clone()))
        .unwrap()
        .get_std_client();

    let ssl = Ssl::new(connector.context()).unwrap();
    let tls_stream = match SslStream::new(ssl, real_stream) {
        Ok(tls_stream) => tls_stream,
        Err(e) => {
            println!("[SHDP:TLS] Error creating TLS stream: {:?}", e);
            return Err(Error {
                code: 0,
                message: e.to_string(),
                kind: ErrorKind::UserDefined(Box::new(e)),
            });
        }
    };

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
        tls_stream,
    )
    .unwrap();

    let _ = handle_connection(Arc::new(Mutex::new(ws_stream))).await;

    Ok(())
}

fn load_connector(cert: Certificate) -> Arc<SslConnector> {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file(&cert.key_path, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(&cert.cert_path).unwrap();

    Arc::new(builder.build())
}
