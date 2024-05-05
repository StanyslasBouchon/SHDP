///
/// Represents a TLS certificate.
///
#[cfg(any(feature = "tls-server", feature = "wss-server"))]
pub struct Certificate {
    /// The path to the certificate file.
    pub cert_path: String,
    /// The path to the private key file.
    pub key_path: String,
}

pub(crate) enum Listener {
    Sync(std::net::TcpListener),
    Async(async_std::net::TcpListener),
}

impl Listener {
    pub(crate) fn get_sync(&self) -> &std::net::TcpListener {
        match self {
            Listener::Sync(listener) => listener,
            _ => panic!("Listener is not a sync listener"),
        }
    }

    pub(crate) fn get_async(&self) -> &async_std::net::TcpListener {
        match self {
            Listener::Async(listener) => listener,
            _ => panic!("Listener is not an async listener"),
        }
    }
}

///
/// Manually stops a connection.
///
/// # Arguments
/// * `port` - The port to stop listening on.
///
/// # Example
/// ```rust,no_run
/// #[cfg(feature = "tcp-server")]
///
///
/// use shdp::prelude::server::stop;
///
/// use shdp::prelude::server::tcp::listen;
///
/// // Creating a server for demonstration purposes.
///
/// listen(String::from("8080"));
///
/// stop(String::from("8080"));
/// ```
pub fn stop(port: String) {
    let mut devices = DEVICES.lock().unwrap();
    if let Some(listener) = devices.remove(&("127.0.0.1".to_string(), port)) {
        match listener {
            Listener::Sync(listener) => drop(listener),
            Listener::Async(listener) => drop(listener),
        };
    }
}

lazy_static! {
    pub(crate) static ref DEVICES: Arc<Mutex<HashMap<RemoteId, Listener>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

#[cfg(any(feature = "tcp-server", feature = "ws-server"))]
pub(super) fn answer_error(version: u8, error: crate::protocol::errors::Error) -> Vec<u8> {
    use bitvec::order::Lsb0;

    use crate::protocol::{
        managers::bits::encoder::FrameEncoder, server::versions::v1::c0x0002::ErrorResponse,
    };

    let mut encoder = FrameEncoder::<Lsb0>::new(version).unwrap();
    encoder.encode(Box::new(ErrorResponse::new(error))).unwrap()
}

#[cfg(feature = "tcp-server")]
pub mod tcp {
    pub use crate::server::tcp::listen;
}

#[cfg(feature = "ws-server")]
pub mod ws {
    pub use crate::server::ws::listen;
}

#[cfg(feature = "tls-server")]
pub mod tls {
    pub use crate::server::tls::listen;
}

#[cfg(feature = "wss-server")]
pub mod wss {
    pub use crate::server::wss::listen;
}

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

use crate::protocol::prelude::RemoteId;
