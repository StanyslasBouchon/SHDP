//!
//! Defines everything for the 0x0004 event.
//!

use bitvec::order::Msb0;

use crate::protocol::{
    client::bits::utils::{ FyveImpl, OperatingCode },
    prelude::common::{
        bits::{ util::BitReversible, BitDecoder, Frame },
        error::Error,
        event::{ EventDecoder, EventEncoder },
    },
};

///
/// Describe an full fyve response.
///
pub struct FullFyveResponse {
    decoder: BitDecoder<Msb0>,
    /// The filename.
    pub filename: String,
    /// The file's content.
    pub content: String,
}

impl FullFyveResponse {
    ///
    /// Creates a new [FullFyveResponse].
    ///
    /// # Arguments
    /// * `decoder` - The [`BitDecoder<Msb0>`] to decode the request.
    ///
    /// # Returns
    /// * [FullFyveResponse] - The created [FullFyveResponse].
    ///
    /// # Example
    /// ```rust
    /// use shdp::prelude::client::versions::v1::r0x0004::FullFyveResponse;
    /// use shdp::prelude::common::bits::BitDecoder;
    /// use bitvec::order::Msb0;
    ///
    /// let decoder = BitDecoder::<Msb0>::new(Vec::new());
    /// let response = FullFyveResponse::new(decoder);
    ///
    /// // These are default values.
    /// assert_eq!(response.filename, String::new());
    /// assert_eq!(response.content, String::new());
    /// ```
    pub fn new(decoder: BitDecoder<Msb0>) -> Self {
        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;21m0x0004\x1b[0m received"
            );
        }

        FullFyveResponse {
            decoder,
            filename: String::new(),
            content: String::new(),
        }
    }
}

impl EventDecoder<Msb0> for FullFyveResponse {
    fn decode(&mut self, frame: Frame<Msb0>) -> Result<(), Error> {
        // Read bytes till hits 0.
        let mut bytes = Vec::<u8>::new();

        loop {
            let byte = self.decoder.read_data(8)? as u8;
            if byte == 0 {
                break;
            }

            bytes.push(byte);
        }

        self.filename = String::from_utf8(bytes).unwrap();

        let mut content = String::new();

        loop {
            if self.decoder.position >= (frame.data_size + 56).into() {
                break;
            }

            let op = FyveImpl::get_op(&mut self.decoder)?;

            if op.kind == OperatingCode::Character {
                let char = op.get_char()?;
                content.push(char);
            } else {
                return Err(Error {
                    code: 400,
                    message: format!("Invalid operation: {:?}", op),
                    kind: crate::protocol::errors::ErrorKind::BadRequest,
                });
            }
        }

        self.content = content;

        Ok(())
    }

    fn get_responses(
        &self
    ) -> Result<
        Vec<Box<dyn EventEncoder<<Msb0 as BitReversible>::Opposite>>>,
        Error
    > {
        Ok(Vec::new())
    }
}
