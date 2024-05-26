//!
//! Defines everything for the 0x0006 event.
//!

use bitvec::order::Lsb0;
use serde_json::Value;

use crate::protocol::prelude::common::{
    bits::{util::BitReversible, BitDecoder, Frame},
    error::Error,
    event::{EventDecoder, EventEncoder},
};

///
/// Describe an interaction response.
///
pub struct InteractionResponse {
    decoder: BitDecoder<Lsb0>,
    /// The request ID.
    pub request_id: u64,
    /// The file's content.
    pub response: Option<Value>,
}

impl InteractionResponse {
    ///
    /// Creates a new [InteractionResponse].
    ///
    /// # Arguments
    /// * `decoder` - The [`BitDecoder<Lsb0>`] to decode the request.
    ///
    /// # Returns
    /// * [InteractionResponse] - The created [InteractionResponse].
    ///
    /// # Example
    /// ```rust
    /// use shdp::prelude::client::versions::v1::r0x0006::InteractionResponse;
    /// use shdp::prelude::common::bits::BitDecoder;
    /// use bitvec::order::Lsb0;
    ///
    /// let decoder = BitDecoder::<Lsb0>::new(Vec::new());
    /// let response = InteractionResponse::new(decoder);
    ///
    /// // These are default values.
    /// assert_eq!(response.request_id, 0);
    /// assert_eq!(response.response, None);
    /// ```
    pub fn new(decoder: BitDecoder<Lsb0>) -> Self {
        if cfg!(feature = "debug") {
            println!("[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;21m0x0006\x1b[0m received");
        }

        InteractionResponse {
            decoder,
            request_id: 0,
            response: None,
        }
    }
}

impl EventDecoder<Lsb0> for InteractionResponse {
    fn decode(&mut self, _: Frame<Lsb0>) -> Result<(), Error> {
        let upper_id = self.decoder.read_data(32)? >> 0;
        let lower_id = self.decoder.read_data(32)? >> 0;
        self.request_id = (u64::from(upper_id) << 32) + u64::from(lower_id);

        // Read bytes till the end.
        let mut bytes = Vec::<u8>::new();

        for _ in 0..self.decoder.frame.len() / 8 {
            bytes.push(self.decoder.read_data(8)? as u8);
        }

        let data = String::from_utf8(bytes).unwrap();

        self.response = Some(match serde_json::from_str(&data) {
            Ok(value) => value,
            Err(_) => Value::Null,
        });

        Ok(())
    }

    fn get_responses(
        &self,
    ) -> Result<Vec<Box<dyn EventEncoder<<Lsb0 as BitReversible>::Opposite>>>, Error> {
        Ok(Vec::new())
    }
}
