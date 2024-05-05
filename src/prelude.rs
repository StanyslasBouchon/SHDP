#[cfg(any(
    feature = "tcp-server",
    feature = "ws-server",
    feature = "tls-server",
    feature = "wss-server"
))]
pub mod server {
    pub use crate::server::prelude::*;
}

#[cfg(any(
    feature = "tcp-client",
    feature = "ws-client",
    feature = "tls-client",
    feature = "wss-client"
))]
pub mod client {
    pub use crate::client::prelude::*;
}

pub mod common {
    pub use crate::protocol::prelude::common::*;
    pub use crate::protocol::prelude::RemoteId;
}
