use std::{
    fs::File,
    io::{self, BufReader},
    sync::Arc,
};

use async_std::net::TcpListener;
use async_tls::TlsAcceptor;
use async_tungstenite::accept_async;
use rustls::{Certificate as RustlsCertificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, read_one, Item};
use tokio::sync::Mutex;
use tungstenite::Error;

use crate::{
    protocol::errors::{Error as ShdpError, ErrorKind},
    server::{
        prelude::{Listener, DEVICES},
        ws::handle_connection,
    },
};

use super::prelude::Certificate;

///
/// Listens for incoming Secure WebSocket connections.
///
/// It creates a new thread for SHDP clients.
///
/// # Arguments
/// * `port` - The port to listen on.
/// * `cert` - The certificate to use.
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
/// use shdp::prelude::server::wss::listen;
///
/// #[tokio::main]
/// async fn main() {
///     let cert = shdp::prelude::server::Certificate {
///         cert_path: String::from("cert.pem"),
///         key_path: String::from("key.pem"),
///     };
///
///     match listen(String::from("8080"), cert).await {
///         Ok(_) => println!("Listening on port 8080"),
///         Err(e) => println!("Error: {:?}", e),
///     }
/// }
/// ```
pub async fn listen(port: String, cert: Certificate) -> Result<(), ShdpError> {
    let acceptor = match load_acceptor(cert) {
        Ok(acceptor) => acceptor,
        Err(e) => {
            return Err(ShdpError {
                code: 500,
                message: format!("Error loading acceptor: {}", e),
                kind: ErrorKind::InternalServerError,
            })
        }
    };

    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)).await {
        Ok(listener) => listener,
        Err(e) => {
            return Err(ShdpError {
                code: 0b1111,
                message: format!("Error binding to port: {}", e),
                kind: ErrorKind::Conflict,
            })
        }
    };

    DEVICES.lock().unwrap().insert(
        ("127.0.0.1".to_string(), port.clone()),
        Listener::Async(listener),
    );

    println!("[SHDP:WS] Listening on port {}", port);

    while let Ok((stream, _)) = DEVICES
        .lock()
        .unwrap()
        .get(&("127.0.0.1".to_string(), port.clone()))
        .unwrap()
        .get_async()
        .accept()
        .await
    {
        let acceptor = acceptor.clone();
        let handle = accept_async(match acceptor.accept(stream.clone()).await {
            Ok(stream) => stream,
            Err(e) => {
                println!("[SHDP:WS] Error accepting TLS connection: {}", e);
                continue;
            }
        })
        .await;

        match handle {
            Ok(ws_stream) => {
                async_std::task::spawn_local(async move {
                    handle_connection(Arc::new(Mutex::new(ws_stream))).await;
                });
            }
            Err(e) => {
                println!("[SHDP:WS] Error accepting WebSocket connection: {}", e);
            }
        }

        if stream.peer_addr().is_ok() {
            println!(
                "[SHDP:WS] New connection from {}",
                stream.peer_addr().unwrap()
            );
        }
    }

    Ok(())
}

fn load_certs(path: &str) -> io::Result<Vec<RustlsCertificate>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = certs(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "could not read certs"))?
        .into_iter()
        .map(RustlsCertificate)
        .collect();
    Ok(certs)
}

fn load_private_key(path: &str) -> io::Result<PrivateKey> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let keys = read_one(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "could not read private key"))?
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no keys found in file"))?;

    match keys {
        Item::RSAKey(data) | Item::PKCS8Key(data) => Ok(PrivateKey(data)),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "unexpected key format",
        )),
    }
}

/// Loads a TLS acceptor from the given certificate configuration.
fn load_acceptor(cert: Certificate) -> Result<TlsAcceptor, Error> {
    let certs = load_certs(&cert.cert_path)?;
    let key = load_private_key(&cert.key_path)?;

    let config: ServerConfig = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| Error::Io(io::Error::new(io::ErrorKind::InvalidInput, e)))?;

    Ok(TlsAcceptor::from(Arc::new(config)))
}
