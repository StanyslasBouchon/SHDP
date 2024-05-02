use crate::protocol::{
    errors::Error,
    server::{bits::builder::InBuilder, event::EventBuilder},
};

pub struct ErrorResponse {
    builder: InBuilder,
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
            builder: InBuilder::new(),
            error,
        }
    }
}

impl EventBuilder for ErrorResponse {
    fn construct(&mut self) -> Result<(), Error> {
        match self.builder.add_data(self.error.code, 16) {
            Err(e) => panic!("Error adding data: {}", e),
            _ => (),
        }

        match self.builder.add_data(0, 8) {
            Err(e) => panic!("Error adding data: {}", e),
            _ => (),
        }

        match self.builder.add_bytes(self.error.message.as_bytes()) {
            Err(e) => panic!("Error adding bytes: {}", e),
            _ => (),
        }

        Ok(())
    }

    fn get_builder(&self) -> &InBuilder {
        &self.builder
    }

    fn get_event(&self) -> u16 {
        0x0002
    }
}
