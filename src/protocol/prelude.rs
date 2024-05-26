pub mod common {
    pub use crate::protocol::args::Arg;
    pub use crate::protocol::versions::Version;

    pub mod utils {
        //!
        //! The utils module contains the utility functions used by the SHDP protocol.
        //! It is used to handle utility functions that may be used by the protocol.
        //!

        ///
        /// Represents a TLS certificate.
        ///
        #[cfg(any(
            feature = "tls-server",
            feature = "wss-server",
            feature = "tls-client",
            feature = "wss-client"
        ))]
        pub struct Certificate {
            /// The path to the certificate file.
            pub cert_path: String,
            /// The path to the private key file.
            pub key_path: String,
        }

        #[derive(Debug)]
        pub(crate) enum Listener<'a> {
            #[cfg(feature = "tcp-server")]
            TokioServer(&'a tokio::net::TcpListener),
            #[cfg(feature = "ws-server")]
            StdServer(async_std::net::TcpListener),
            #[cfg(feature = "tcp-client")]
            TokioClient(&'a mut tokio::net::TcpStream),
            #[cfg(feature = "ws-client")]
            StdClient(&'a mut std::net::TcpStream),
            #[cfg(feature = "ws-server")]
            _Phantom(PhantomData<&'a ()>),
        }

        impl<'a> Listener<'a> {
            #[cfg(feature = "tcp-server")]
            pub(crate) fn get_tokio_server(&self) -> &tokio::net::TcpListener {
                match self {
                    Listener::TokioServer(listener) => listener,
                    #[cfg(feature = "ws-server")]
                    _ => panic!("Listener is not a tokio listener"),
                }
            }

            #[cfg(feature = "ws-server")]
            pub(crate) fn get_std_server(&self) -> &async_std::net::TcpListener {
                match self {
                    Listener::StdServer(listener) => listener,
                    _ => panic!("Listener is not an async_std listener"),
                }
            }

            #[cfg(feature = "tcp-client")]
            pub(crate) fn get_tokio_client(&mut self) -> &mut tokio::net::TcpStream {
                match self {
                    Listener::TokioClient(listener) => listener,
                    #[cfg(feature = "ws-client")]
                    _ => panic!("Listener is not a tokio stream"),
                }
            }

            #[cfg(feature = "ws-client")]
            pub(crate) fn get_std_client(&mut self) -> &mut std::net::TcpStream {
                match self {
                    Listener::StdClient(listener) => listener,
                    #[cfg(feature = "tcp-client")]
                    _ => panic!("Listener is not an std stream"),
                }
            }
        }

        ///
        /// Manually stops a connection.
        ///
        /// # Arguments
        /// * `ip` - The ip address to stop listening on.
        /// * `port` - The port to stop listening on.
        ///
        /// # Example
        /// ```rust,no_run
        /// #[cfg(feature = "tcp-server")]
        ///
        ///
        /// use shdp::prelude::common::utils::stop;
        ///
        /// use shdp::prelude::server::tcp::listen;
        ///
        /// // Creating a server for demonstration purposes.
        ///
        /// listen(String::from("8080"));
        ///
        /// stop(String::from("127.0.0.1"), String::from("8080"));
        /// ```
        pub async fn stop(ip: String, port: String) {
            let mut devices = DEVICES.lock().unwrap();
            if let Some(listener) = devices.remove(&(ip, port)) {
                match listener {
                    #[cfg(feature = "tcp-server")]
                    Listener::TokioServer(listener) => drop(listener.to_owned()),
                    #[cfg(feature = "ws-server")]
                    Listener::StdServer(listener) => drop(listener),
                    #[cfg(feature = "tcp-client")]
                    Listener::TokioClient(listener) => {
                        let _ = listener.shutdown().await;
                    }
                    #[cfg(feature = "ws-client")]
                    Listener::StdClient(listener) => {
                        let _ = listener.shutdown(std::net::Shutdown::Both);
                    }
                    #[cfg(feature = "ws-server")]
                    Listener::_Phantom(_) => {
                        panic!("Phantom should not be used!")
                    }
                };
            }
        }

        lazy_static! {
            pub(crate) static ref DEVICES: Arc<Mutex<HashMap<RemoteId, Listener<'static>>>> =
                Arc::new(Mutex::new(HashMap::new()));
        }

        #[cfg(feature = "ws-server")]
        use std::marker::PhantomData;

        use std::{
            collections::HashMap,
            sync::{Arc, Mutex},
        };

        use lazy_static::lazy_static;
        #[cfg(feature = "tcp-client")]
        use tokio::io::AsyncWriteExt;

        use crate::protocol::prelude::RemoteId;
    }

    pub mod bits {
        //!
        //! The bits module contains the bit manipulation functions and structures used by the SHDP protocol.
        //! It is used to handle the bits that are sent and received by the protocol.
        //!
        pub use crate::protocol::managers::bits::decoder::*;
        pub use crate::protocol::managers::bits::encoder::*;

        pub mod util {
            //!
            //! The util module contains the utility functions used by the SHDP protocol.
            //! It is used to handle utility functions that may be used by the protocol.
            //!

            pub use crate::protocol::managers::bits::prelude::*;
        }
    }

    pub mod event {
        //!
        //! The event module contains the event structures and functions used by the SHDP protocol.
        //! It is used to handle events that may occur during the protocol execution.
        //!

        pub use crate::protocol::managers::event::*;
    }

    pub mod error {
        //!
        //! The error module contains the error structures and functions used by the SHDP protocol.
        //! It is used to handle errors that may occur during the protocol execution.
        //!
        pub use crate::protocol::errors::*;
    }

    pub mod registry {
        //!
        //! The registry module contains the registry structures and functions used by the SHDP protocol.
        //! It is used to store events and listeners that will be by each the server and the client.
        //!
        pub use crate::protocol::managers::registry::*;
    }
}

///
/// A RemoteId is a tuple of two strings, the first being the remote's host address and the second being the remote's port.
///
pub type RemoteId = (String, String);
