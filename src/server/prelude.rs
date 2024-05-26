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
    //! The TCP server module.
    pub use crate::server::tcp::listen;
}

#[cfg(feature = "ws-server")]
pub mod ws {
    //! The WebSocket server module.
    pub use crate::server::ws::listen;
}

#[cfg(feature = "tls-server")]
pub mod tls {
    //! The TLS server module.
    pub use crate::server::tls::listen;
}

#[cfg(feature = "wss-server")]
pub mod wss {
    //! The Secured WebSocket server module.
    pub use crate::server::wss::listen;
}

pub mod versions {
    //! The versions module contains the different versions of the SHDP protocol.
    pub mod v1 {
        //! The v1 module contains the version 1 of the SHDP protocol.
        pub use crate::protocol::server::versions::v1::*;
    }
}
