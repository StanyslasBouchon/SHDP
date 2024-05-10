use bitvec::order::Msb0;

use crate::protocol::prelude::common::{
    bits::BitEncoder,
    error::Error,
    event::EventEncoder,
};

pub struct ComponentNeedsRequest {
    encoder: BitEncoder<Msb0>,
    component_name: String,
}

impl ComponentNeedsRequest {
    pub fn new(component_name: String) -> Self {
        if cfg!(feature = "debug") {
            println!("[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;21m0x0000\x1b[0m created ({})", component_name);
        }

        ComponentNeedsRequest {
            encoder: BitEncoder::<Msb0>::new(),
            component_name,
        }
    }
}

impl EventEncoder<Msb0> for ComponentNeedsRequest {
    fn encode(&mut self) -> Result<(), Error> {
        self.encoder.add_bytes(self.component_name.as_bytes())?;

        Ok(())
    }

    fn get_encoder(&self) -> &BitEncoder<Msb0> {
        &self.encoder
    }

    fn get_event(&self) -> u16 {
        0x0000
    }
}
