// use std::sync::{Arc, Mutex};

// use openssl::ssl::{Ssl, SslConnector, SslFiletype, SslMethod};
// use tokio::net::TcpStream;
// use tokio_openssl::SslStream;

// use crate::{
//     client::tcp::handle_client,
//     protocol::prelude::common::{
//         error::{Error, ErrorKind},
//         utils::{Certificate, Listener, DEVICES},
//     },
// };

// ///
// /// Listens for incoming TLS connections.
// ///
// /// It creates a new thread for SHDP clients.
// ///
// /// # Arguments
// /// * `port` - The port to listen on.
// /// * `cert` - The certificate to use.
// ///
// /// # Returns
// /// * [Result<(), Error>] - The result of the operation.
// ///
// /// # Errors
// /// Generated errors are related to the I/O operations.<br>
// /// They need to be handled by the caller.
// ///
// /// # Example
// /// ```rust,no_run
// /// use shdp::prelude::client::tls::connect;
// /// use shdp::prelude::common::utils::Certificate;
// ///
// /// #[tokio::main]
// /// async fn main() {
// ///     match connect(String::from("157.165.164.160"), String::from("8080"), Certificate {
// ///         cert_path: String::from("cert.pem"),
// ///         key_path: String::from("key.pem"),
// ///     }).await {
// ///         Ok(_) => println!("Connected established"),
// ///         Err(e) => println!("Error: {:?}", e),
// ///     }
// /// }
// /// ```
// pub async fn connect(ip: String, port: String, cert: Certificate) -> Result<(), Error> {
//     let connector = load_connector(cert);
//     let stream = match TcpStream::connect(format!("{}:{}", ip, port)).await {
//         Ok(stream) => stream,
//         Err(e) => {
//             println!(
//                 "[SHDP:TLS] Error connecting to {}:{}",
//                 ip.clone(),
//                 port.clone()
//             );
//             return Err(Error {
//                 code: 0,
//                 message: e.to_string(),
//                 kind: ErrorKind::UserDefined(Box::new(e)),
//             });
//         }
//     };

//     let static_stream = Arc::new(Mutex::new(stream));

//     DEVICES.lock().unwrap().insert(
//         (ip.clone(), port.clone()),
//         Listener::TokioClient(static_stream),
//     );

//     let mut devices = DEVICES.lock().unwrap();
//     let real_stream = devices
//         .get_mut(&(ip.clone(), port.clone()))
//         .unwrap()
//         .get_tokio_client();

//     // let ssl = Ssl::new(connector.context()).unwrap();
//     // let mut tls_stream = match SslStream::new(ssl, real_stream.lock().unwrap().) {
//     //     Ok(tls_stream) => Arc::new(Mutex::new(tls_stream)),
//     //     Err(e) => {
//     //         println!("[SHDP:TLS] Error creating TLS stream: {:?}", e);
//     //         return Err(Error {
//     //             code: 0,
//     //             message: e.to_string(),
//     //             kind: ErrorKind::UserDefined(Box::new(e)),
//     //         });
//     //     }
//     // };

//     // println!("[SHDP:TLS] Connected to {}:{}", ip.clone(), port.clone());

//     // let _ = handle_client(tls_stream).await;

//     Ok(())
// }

// fn load_connector(cert: Certificate) -> Arc<SslConnector> {
//     let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();

//     builder
//         .set_private_key_file(&cert.key_path, SslFiletype::PEM)
//         .unwrap();
//     builder.set_certificate_chain_file(&cert.cert_path).unwrap();

//     Arc::new(builder.build())
// }
