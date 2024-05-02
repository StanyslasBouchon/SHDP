use serde_json::Value;

use crate::protocol::{
    errors::Error,
    server::{bits::builder::InBuilder, event::EventBuilder},
};

pub struct InteractionResponse {
    builder: InBuilder,
    request_id: u64,
    response: Option<Value>,
}

impl InteractionResponse {
    pub fn new(request_id: u64, response: Option<Value>) -> Self {
        if cfg!(feature = "debug") {
            println!("[\x1b[38;5;227mSHDP\x1b[0m] \x1b[38;5;163m0x0006\x1b[0m created");
        }

        InteractionResponse {
            builder: InBuilder::new(),
            request_id,
            response,
        }
    }
}

impl EventBuilder for InteractionResponse {
    fn construct(&mut self) -> Result<(), Error> {
        let upper_request_id = (self.request_id >> 32) as u32;
        let lower_request_id = (self.request_id & 0xFFFFFFFF) as u32;

        self.builder.add_data(upper_request_id, 32)?;
        self.builder.add_data(lower_request_id, 32)?;

        match &self.response {
            Some(response) => {
                self.builder.add_bytes(response.to_string().as_bytes())?;
            }
            _ => (),
        }

        Ok(())
    }

    fn get_builder(&self) -> &InBuilder {
        &self.builder
    }

    fn get_event(&self) -> u16 {
        0x0006
    }
}
