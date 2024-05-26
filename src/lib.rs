#![feature(doc_auto_cfg)]
#![warn(missing_docs)]

//! ## Features
//!
//! By default, SHDP is compiled without any optional features except for `tcp-server`.
//!
//! | Feature      | Description                                               |
//! |--------------|-----------------------------------------------------------|
//! | `serde`      | Support for JSON (de)serialization.                       |
//! | `tcp-server` | Support for server TCP connections. (enabled by default). |
//! | `tcp-client` | Support for client TCP connections.                       |
//! | `ws-server`  | Support for server WebSocket connections                  |
//! | `ws-client`  | Support for client WebSocket connections.                 |
//! | `wss-server` | Support for server Secured WebSocket connections.         |
//! | `wss-client` | Support for client Secured WebSocket connections.         |
//! | `tls-server` | Support for server TLS connections.                       |
//! | `tls-client` | Support for client TLS connections.                       |
//! | `debug`      | Enable debug logs.                                        |
//!

pub(crate) mod protocol;

#[cfg(any(feature = "tcp-client", feature = "ws-client"))]
pub(crate) mod client;

#[cfg(any(feature = "tcp-server", feature = "ws-server"))]
pub(crate) mod server;

#[cfg(test)]
mod tests;

pub mod prelude;
