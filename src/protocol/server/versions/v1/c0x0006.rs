use bitvec::order::Lsb0;
use serde_json::Value;

use crate::protocol::{
    errors::Error,
    managers::{bits::encoder::BitEncoder, event::EventEncoder},
};

pub struct InteractionResponse {
    encoder: BitEncoder<Lsb0>,
    request_id: u64,
    response: Option<Value>,
}

impl InteractionResponse {
    pub fn new(request_id: u64, response: Option<Value>) -> Self {
        if cfg!(feature = "debug") {
            println!("[\x1b[38;5;227mSHDP\x1b[0m] \x1b[38;5;163m0x0006\x1b[0m created");
        }

        InteractionResponse {
            encoder: BitEncoder::<Lsb0>::new(),
            request_id,
            response,
        }
    }
}

impl EventEncoder<Lsb0> for InteractionResponse {
    fn encode(&mut self) -> Result<(), Error> {
        let upper_request_id = (self.request_id >> 32) as u32;
        let lower_request_id = (self.request_id & 0xFFFFFFFF) as u32;

        self.encoder.add_data(upper_request_id, 32)?;
        self.encoder.add_data(lower_request_id, 32)?;

        match &self.response {
            Some(response) => {
                self.encoder.add_bytes(response.to_string().as_bytes())?;
            }
            _ => (),
        }

        Ok(())
    }

    fn get_encoder(&self) -> &BitEncoder<Lsb0> {
        &self.encoder
    }

    fn get_event(&self) -> u16 {
        0x0006
    }
}
