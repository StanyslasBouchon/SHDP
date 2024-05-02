mod protocol;

#[cfg(any(feature = "tcp-client", feature = "ws-client"))]
pub mod client;

#[cfg(any(feature = "tcp-server", feature = "ws-server"))]
pub mod server;
