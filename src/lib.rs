pub(crate) mod protocol;

#[cfg(any(feature = "tcp-client", feature = "ws-client"))]
pub(crate) mod client;

#[cfg(any(feature = "tcp-server", feature = "ws-server"))]
pub(crate) mod server;

#[cfg(test)]
mod tests;

pub mod prelude;
