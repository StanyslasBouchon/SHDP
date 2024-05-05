use bitvec::order::{Lsb0, Msb0};
use serde_json::Value;

use crate::protocol::{
    errors::{Error, ErrorKind},
    managers::{
        bits::decoder::BitDecoder,
        event::{EventDecoder, EventEncoder},
    },
    prelude::common::registry::EVENT_REGISTRY_MSB,
};

use super::c0x0006::InteractionResponse;

#[derive(Clone)]
pub struct InteractionRequest {
    decoder: BitDecoder<Msb0>,
    pub request_id: u64,
    pub function_name: String,
    pub parent_name: String,
    pub object_id: Option<i32>,
    pub params: Option<Value>,
    pub token: Option<String>,
}

impl InteractionRequest {
    pub fn new(decoder: BitDecoder<Msb0>) -> Self {
        if cfg!(feature = "debug") {
            println!("[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;125m0x0005\x1b[0m received");
        }

        InteractionRequest {
            decoder,
            request_id: 0,
            parent_name: String::new(),
            function_name: String::new(),
            object_id: None,
            params: None,
            token: None,
        }
    }
}

impl EventDecoder<Msb0> for InteractionRequest {
    fn decode(&mut self) -> Result<(), Error> {
        let upper_request_id = self.decoder.read_data(32)?;
        let lower_request_id = self.decoder.read_data(32)?;
        self.request_id = (upper_request_id as u64) << 32 | lower_request_id as u64;

        let bit_length = self.decoder.frame.len() - 64;
        let byte_length = bit_length / 8;
        let mut data = Vec::<u8>::new();

        for _ in 0..byte_length {
            let byte = self.decoder.read_data(8)? as u8;
            data.push(byte);
        }

        let string: String = data.iter().map(|&b| b as char).collect();
        let parts: Vec<&str> = string.split('\x00').collect();

        self.function_name = String::from(parts[0]);
        self.parent_name = String::from(parts[1]);

        if self.function_name.is_empty() {
            return Err(Error {
                code: 400,
                message: "Function name is empty".to_string(),
                kind: ErrorKind::BadRequest,
            });
        }

        if self.parent_name.is_empty() {
            return Err(Error {
                code: 400,
                message: "Table name is empty".to_string(),
                kind: ErrorKind::BadRequest,
            });
        }

        self.token = if parts[2].is_empty() {
            None
        } else {
            Some(parts[2].to_string())
        };
        self.object_id = if parts[3].is_empty() {
            None
        } else {
            Some(parts[3].to_string().parse::<i32>().unwrap())
        };
        self.params = if parts[4].is_empty() {
            None
        } else {
            Some(serde_json::from_str(parts[4]).unwrap())
        };

        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;125m0x0005\x1b[0m: function_name: {}, table: {}, object_id: {:?}, params: {:?}, token: {:?}",
                self.function_name, self.parent_name, self.object_id, self.params, self.token
            );
        }

        Ok(())
    }

    fn get_responses(&self) -> Result<Vec<Box<dyn EventEncoder<Lsb0>>>, Error> {
        let args = match EVENT_REGISTRY_MSB.lock().unwrap().get_listener((1, 0x0005)) {
            Some(listener) => listener(Box::new(self.clone())),
            None => {
                return Err(Error {
                    code: 404,
                    message: "Listener not found for 0x0005".to_string(),
                    kind: ErrorKind::NotFound,
                })
            }
        };

        let response = args.get(0).unwrap().to_option_value()?;

        Ok(vec![Box::new(InteractionResponse::new(
            self.request_id,
            response,
        ))])
    }
}
