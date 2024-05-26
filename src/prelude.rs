//!
//! The prelude module contains the re-exports of the most commonly used items in the SHDP protocol.
//! It is used to simplify the usage of the protocol.
//!

#[cfg(any(feature = "tcp-server", feature = "ws-server"))]
pub mod server {
    //! The server module contains the server implementations for the SHDP protocol.
    //!
    //! It is automatically included when the `tcp-server`, `ws-server`, `tls-server`, or `wss-server` features are enabled.

    pub use crate::server::prelude::*;
}

#[cfg(any(feature = "tcp-client", feature = "ws-client"))]
pub mod client {
    //! The client module contains the client implementations for the SHDP protocol.
    //!
    //! It is automatically included when the `tcp-client`, `ws-client`, `tls-client`, or `wss-client` features are enabled.

    pub use crate::client::prelude::*;
}

pub mod common {
    //! The common module contains the common structures and functions used by the SHDP protocol.
    //!
    //! It is included by default.

    pub use crate::protocol::prelude::common::*;
    pub use crate::protocol::prelude::RemoteId;
}
