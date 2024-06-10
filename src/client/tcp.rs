// use std::{
//     sync::{Arc, Mutex},
//     time::Duration,
// };

// use bitvec::order::{Lsb0, Msb0};
// use tokio::{
//     io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
//     net::TcpStream,
//     time::timeout,
// };

// use crate::protocol::{
//     managers::bits::{
//         decoder::{BitDecoder, FrameDecoder},
//         encoder::FrameEncoder,
//     },
//     prelude::common::{
//         error::{Error, ErrorKind},
//         event::EventEncoder,
//         registry::EVENT_REGISTRY_MSB,
//         utils::{Listener, DEVICES},
//     },
// };

// ///
// /// Listens for incoming TCP connections.
// ///
// /// It creates a new thread for SHDP clients.
// ///
// /// # Arguments
// /// * `port` - The port to listen on.
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
// /// use shdp::prelude::client::tcp::connect;
// ///
// /// #[tokio::main]
// /// async fn main() {
// ///     match connect(String::from("157.165.164.160"), String::from("8080")).await {
// ///         Ok(_) => println!("Connection established"),
// ///         Err(e) => println!("Error: {:?}", e),
// ///     }
// /// }
// ///
// /// ```
// pub async fn connect(ip: String, port: String) -> Result<(), Error> {
//     println!("Connecting to {}:{}", ip.clone(), port.clone());
//     let connection_result = timeout(
//         Duration::from_secs(5),
//         TcpStream::connect(format!("{}:{}", ip, port)),
//     )
//     .await;

//     println!("ddsd");

//     let stream = match connection_result {
//         Ok(Ok(stream)) => {
//             println!("[SHDP:TCP] Connected to {}:{}", ip, port);
//             stream
//         }
//         Ok(Err(e)) => {
//             println!("[SHDP:TCP] Error connecting to {}:{} - {}", ip, port, e);
//             return Err(Error {
//                 code: 0,
//                 message: e.to_string(),
//                 kind: ErrorKind::UserDefined(Box::new(e)),
//             });
//         }
//         Err(_) => {
//             println!("[SHDP:TCP] Timeout connecting to {}:{}", ip, port);
//             return Err(Error {
//                 code: 0,
//                 message: "Connection attempt timed out".to_string(),
//                 kind: ErrorKind::Expired,
//             });
//         }
//     };

//     let static_stream = Arc::new(Mutex::new(stream));

//     DEVICES.lock().unwrap().insert(
//         (ip.clone(), port.clone()),
//         Listener::TokioClient(static_stream.clone()),
//     );

//     println!("[SHDP:TCP] Connected to {}:{}", ip.clone(), port.clone());

//     let mut devices = DEVICES.lock().unwrap();
//     let real_stream = devices
//         .get_mut(&(ip.clone(), port.clone()))
//         .unwrap()
//         .get_tokio_client();

//     let _ = handle_client(real_stream).await;

//     Ok(())
// }

// ///
// /// Sends a raw event to a SHDP client.
// ///
// /// # Arguments
// /// * `to` - The IP and port of the client.
// /// * `event` - The event to send.
// ///
// /// # Returns
// /// * [Result<(), Error>] - The result of the operation.
// ///
// /// # Example
// /// ```rust,no_run
// /// use shdp::prelude::client::tcp::send_raw_event;
// /// use shdp::prelude::common::bits::FrameEncoder;
// /// use shdp::prelude::common::error::Error;
// /// use shdp::prelude::common::event::EventEncoder;
// /// use bitvec::order::Lsb0;
// ///
// /// #[tokio::main]
// /// async fn main() {
// ///     let event = Box::new(FrameEncoder::<Lsb0>::new(1).unwrap());
// ///     match send_raw_event(
// ///         (String::from("157.165.164.160"), String::from("8080")),
// ///         event
// ///     ).await {
// ///         Ok(_) => println!("Event sent"),
// ///         Err(e) => println!("Error: {:?}", e),
// ///     }
// /// }
// /// ```
// ///
// pub async fn send_raw_event(
//     to: (String, String),
//     event: Box<dyn EventEncoder<Lsb0>>,
// ) -> Result<(), Error> {
//     let mut devices = DEVICES.lock().unwrap();
//     let stream = devices.get_mut(&to).unwrap().get_tokio_client();

//     let mut encoder = match FrameEncoder::<Lsb0>::new(1) {
//         Ok(encoder) => encoder,
//         Err(e) => {
//             println!("[SHDP:TCP] Error creating encoder: {}", e);
//             return Err(e);
//         }
//     };

//     let frame = encoder.encode(event).unwrap();

//     match stream.lock().unwrap().write_all(&frame).await {
//         Ok(_) => (),
//         Err(e) => {
//             println!("[SHDP:TCP] Error writing to stream: {}", e);
//             return Err(Error {
//                 code: 0,
//                 message: e.to_string(),
//                 kind: ErrorKind::UserDefined(Box::new(e)),
//             });
//         }
//     }

//     Ok(())
// }

// pub async fn handle_client<IO: AsyncRead + AsyncWrite + Unpin>(
//     stream: Arc<std::sync::Mutex<IO>>,
// ) -> Result<(), Error> {
//     loop {
//         let mut buffer = [0u8; 512];
//         let size = match stream.lock().unwrap().read(&mut buffer).await {
//             Ok(0) => {
//                 break;
//             }
//             Ok(size) => size,
//             Err(e) => {
//                 println!("[SHDP:TCP] Error reading from stream: {}", e);
//                 break;
//             }
//         };

//         let mut decoder = BitDecoder::<Msb0>::new(buffer[..size].to_vec());
//         let mut frame_decoder = FrameDecoder::<Msb0>::new(decoder);
//         let data = frame_decoder.decode().unwrap();
//         decoder = frame_decoder.get_decoder().to_owned();

//         let registry = EVENT_REGISTRY_MSB.lock().unwrap();
//         let factory = match registry.get_event((data.version, data.event)) {
//             Some(event) => event,
//             None => {
//                 println!(
//                     "[SHDP:TCP] Event not found: {} <-> {}",
//                     data.version, data.event
//                 );

//                 return Err(Error {
//                     code: 404,
//                     message: "Event not found".to_string(),
//                     kind: crate::protocol::errors::ErrorKind::NotFound,
//                 });
//             }
//         };

//         let mut event = factory(decoder);
//         event.decode(data.clone())?;

//         let responses = event.get_responses()?;

//         for response in responses {
//             let mut encoder = match FrameEncoder::<Lsb0>::new(data.version) {
//                 Ok(encoder) => encoder,
//                 Err(e) => {
//                     println!("[SHDP:TCP] Error creating encoder: {}", e);
//                     return Err(e);
//                 }
//             };

//             let frame = encoder.encode(response).unwrap();

//             match stream.lock().unwrap().write_all(&frame).await {
//                 Ok(_) => (),
//                 Err(e) => {
//                     println!("[SHDP:TCP] Error writing to stream: {}", e);
//                     return Err(Error {
//                         code: 0,
//                         message: e.to_string(),
//                         kind: ErrorKind::UserDefined(Box::new(e)),
//                     });
//                 }
//             }
//         }
//     }

//     println!("[SHDP:TCP] Connection closed");

//     Ok(())
// }
