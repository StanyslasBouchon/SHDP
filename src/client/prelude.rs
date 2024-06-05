pub mod versions {
    //! This module contains the different versions of the client protocol.
    pub mod v1 {
        //! This module contains the v1 version of the client protocol.
        pub use crate::protocol::client::versions::v1::*;
    }
}

#[cfg(feature = "tcp-client")]
pub mod tcp {
    //! The TCP client module.
    pub use crate::client::tcp::connect;
    pub use crate::client::tcp::send_raw_event;
}

#[cfg(feature = "ws-client")]
pub mod ws {
    //! The WebSocket client module.
    pub use crate::client::ws::connect;
}

#[cfg(feature = "tls-client")]
pub mod tls {
    //! The TLS client module.
    pub use crate::client::tls::connect;
}

#[cfg(feature = "wss-client")]
pub mod wss {
    //! The Secured WebSocket client module.
    pub use crate::client::wss::connect;
}
