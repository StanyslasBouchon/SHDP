//!
//! Defines everything for the 0x0000 event.
//!

use std::path::Path;

use bitvec::order::{Lsb0, Msb0};

use crate::protocol::{
    errors::{Error, ErrorKind},
    managers::{
        bits::decoder::BitDecoder,
        event::{EventDecoder, EventEncoder},
    },
    prelude::common::{bits::Frame, registry::EVENT_REGISTRY_MSB},
    server::versions::v1::c0x0003::ComponentNeedsResponse,
};

use super::{c0x0001::HtmlFileResponse, c0x0004::FullFyveResponse};

///
/// The [ComponentNeedsRequest] struct is used to request a component and its files.
///
/// It is identified as event 0x0000.
///
#[derive(Clone)]
pub struct ComponentNeedsRequest {
    decoder: BitDecoder<Msb0>,
    /// The name of the component to request.
    pub requested_component_name: String,
}

impl ComponentNeedsRequest {
    ///
    /// Creates a new [ComponentNeedsRequest].
    ///
    /// # Arguments
    /// * `decoder` - The [`BitDecoder<Msb0>`] to decode the request.
    ///
    /// # Returns
    /// * [ComponentNeedsRequest] - The created [ComponentNeedsRequest].
    ///
    /// # Example
    /// ```rust
    /// use shdp::prelude::server::versions::v1::r0x0000::ComponentNeedsRequest;
    /// use shdp::prelude::common::bits::BitDecoder;
    /// use bitvec::order::Msb0;
    ///
    /// let decoder = BitDecoder::<Msb0>::new(Vec::new());
    /// let request = ComponentNeedsRequest::new(decoder);
    ///
    /// // These are default values.
    /// assert_eq!(request.requested_component_name, String::new());
    /// ```
    ///
    pub fn new(decoder: BitDecoder<Msb0>) -> Self {
        if cfg!(feature = "debug") {
            println!("[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;21m0x0000\x1b[0m received");
        }

        ComponentNeedsRequest {
            decoder,
            requested_component_name: String::new(),
        }
    }
}

impl EventDecoder<Msb0> for ComponentNeedsRequest {
    fn decode(&mut self, frame: Frame<Msb0>) -> Result<(), Error> {
        // Read bytes till the end.
        let mut bytes = Vec::<u8>::new();

        for _ in 0..(frame.data_size / 8) {
            bytes.push(self.decoder.read_data(8)? as u8);
        }

        self.requested_component_name = String::from_utf8(bytes).unwrap();

        Ok(())
    }

    fn get_responses(&self) -> Result<Vec<Box<dyn EventEncoder<Lsb0>>>, Error> {
        let args = match EVENT_REGISTRY_MSB.lock().unwrap().get_listener((1, 0x0000)) {
            Some(listener) => listener(Box::new(self.clone())),
            None => {
                return Err(Error {
                    code: 404,
                    message: "Listener not found for 0x0000".to_string(),
                    kind: ErrorKind::NotFound,
                });
            }
        };

        let title = args.get(0).unwrap().to_option_text()?;
        let files_path = args.get(1).unwrap().to_vec_text()?;
        let files: Result<Vec<String>, Error> = files_path
            .iter()
            .map(|path| {
                let file_name = Path::new(path)
                    .file_name()
                    .and_then(|p| p.to_str())
                    .map(String::from);
                file_name.ok_or_else(|| Error {
                    code: 400,
                    message: format!("Invalid file name: {}", path),
                    kind: ErrorKind::BadRequest,
                })
            })
            .collect();

        let files = files?;

        let mut responses: Vec<Box<dyn EventEncoder<Lsb0>>> = files_path
            .iter()
            .map(|path| {
                if path.ends_with(".html") {
                    Box::new(HtmlFileResponse::new(path.to_owned())) as Box<dyn EventEncoder<Lsb0>>
                } else {
                    Box::new(FullFyveResponse::new(path.to_owned())) as Box<dyn EventEncoder<Lsb0>>
                }
            })
            .collect();

        responses.insert(
            0,
            Box::new(ComponentNeedsResponse::new(
                self.requested_component_name.clone(),
                title,
                files,
            )),
        );

        Ok(responses)
    }
}
