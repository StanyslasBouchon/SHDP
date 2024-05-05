use bitvec::order::Lsb0;

use crate::protocol::{
    errors::Error,
    managers::{bits::encoder::BitEncoder, event::EventEncoder},
};

pub struct ErrorResponse {
    encoder: BitEncoder<Lsb0>,
    error: Error,
}

impl ErrorResponse {
    pub fn new(error: Error) -> Self {
        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;227mSHDP\x1b[0m] \x1b[38;5;160m0x0002\x1b[0m created (\x1b[38;5;160m{}\x1b[0m): [{}] {}",
                error.code, error.kind, error.message
            );
        }

        ErrorResponse {
            encoder: BitEncoder::<Lsb0>::new(),
            error,
        }
    }
}

impl EventEncoder<Lsb0> for ErrorResponse {
    fn encode(&mut self) -> Result<(), Error> {
        match self.encoder.add_data(self.error.code, 16) {
            Err(e) => panic!("Error adding data: {}", e),
            _ => (),
        }

        match self.encoder.add_data(0, 8) {
            Err(e) => panic!("Error adding data: {}", e),
            _ => (),
        }

        match self.encoder.add_bytes(self.error.message.as_bytes()) {
            Err(e) => panic!("Error adding bytes: {}", e),
            _ => (),
        }

        Ok(())
    }

    fn get_encoder(&self) -> &BitEncoder<Lsb0> {
        &self.encoder
    }

    fn get_event(&self) -> u16 {
        0x0002
    }
}
