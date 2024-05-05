use bitvec::order::Lsb0;

use crate::protocol::{
    errors::Error,
    managers::{bits::encoder::BitEncoder, event::EventEncoder},
};

pub struct ComponentNeedsResponse {
    encoder: BitEncoder<Lsb0>,
    component_name: String,
    title: Option<String>,
    files: Vec<String>,
}

impl ComponentNeedsResponse {
    pub fn new(component_name: String, title: Option<String>, files: Vec<String>) -> Self {
        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;227mSHDP\x1b[0m] \x1b[38;5;192m0x0003\x1b[0m created ({})",
                component_name
            );
        }

        ComponentNeedsResponse {
            encoder: BitEncoder::<Lsb0>::new(),
            component_name,
            title,
            files,
        }
    }
}

impl EventEncoder<Lsb0> for ComponentNeedsResponse {
    fn encode(&mut self) -> Result<(), Error> {
        self.encoder.add_bytes(self.component_name.as_bytes())?;

        match &self.title {
            Some(title) => {
                self.encoder.add_data(0, 8)?;
                self.encoder.add_bytes(title.as_bytes())?;
            }
            None => (),
        }

        if self.files.len() > 0 {
            for file in &self.files {
                self.encoder.add_data(0, 8)?;
                self.encoder.add_bytes(file.as_bytes())?;
            }
        } else {
            self.encoder.add_data(1, 8)?;
        }

        Ok(())
    }

    fn get_encoder(&self) -> &BitEncoder<Lsb0> {
        &self.encoder
    }

    fn get_event(&self) -> u16 {
        0x0003
    }
}
