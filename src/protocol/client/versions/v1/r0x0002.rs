//!
//! Defines everything for the 0x0002 event.
//!

use bitvec::order::Msb0;

use crate::protocol::prelude::common::{
    bits::{ util::BitReversible, BitDecoder, Frame },
    error::Error,
    event::{ EventDecoder, EventEncoder },
};

///
/// Describe an error response.
///
pub struct ErrorResponse {
    decoder: BitDecoder<Msb0>,
    /// The error code.
    pub code: u16,
    /// The error message.
    pub message: String,
}

impl ErrorResponse {
    ///
    /// Creates a new [ErrorResponse].
    ///
    /// # Arguments
    /// * `decoder` - The [`BitDecoder<Msb0>`] to decode the request.
    ///
    /// # Returns
    /// * [ErrorResponse] - The created [ErrorResponse].
    ///
    /// # Example
    /// ```rust
    /// use shdp::prelude::client::versions::v1::r0x0002::ErrorResponse;
    /// use shdp::prelude::common::bits::BitDecoder;
    /// use bitvec::order::Msb0;
    ///
    /// let decoder = BitDecoder::<Msb0>::new(Vec::new());
    /// let response = ErrorResponse::new(decoder);
    ///
    /// // These are default values.
    /// assert_eq!(response.code, 0);
    /// assert_eq!(response.message, String::new());
    /// ```
    pub fn new(decoder: BitDecoder<Msb0>) -> Self {
        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;21m0x0002\x1b[0m received"
            );
        }

        ErrorResponse {
            decoder,
            code: 0,
            message: String::new(),
        }
    }
}

impl EventDecoder<Msb0> for ErrorResponse {
    fn decode(&mut self, frame: Frame<Msb0>) -> Result<(), Error> {
        self.code = self.decoder.read_data(16)? as u16;
        self.decoder.position += 8; // Separator

        // Read bytes till the end.
        let mut bytes = Vec::<u8>::new();

        for _ in 0..(frame.data_size - 24) / 8 {
            bytes.push(self.decoder.read_data(8)? as u8);
        }

        self.message = String::from_utf8(bytes).unwrap();

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
