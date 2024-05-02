use crate::protocol::{
    errors::Error,
    server::{bits::builder::InBuilder, event::EventBuilder},
};

pub struct ComponentNeedsResponse {
    builder: InBuilder,
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
            builder: InBuilder::new(),
            component_name,
            title,
            files,
        }
    }
}

impl EventBuilder for ComponentNeedsResponse {
    fn construct(&mut self) -> Result<(), Error> {
        self.builder.add_bytes(self.component_name.as_bytes())?;

        match &self.title {
            Some(title) => {
                self.builder.add_data(0, 8)?;
                self.builder.add_bytes(title.as_bytes())?;
            }
            None => (),
        }

        if self.files.len() > 0 {
            for file in &self.files {
                self.builder.add_data(0, 8)?;
                self.builder.add_bytes(file.as_bytes())?;
            }
        } else {
            self.builder.add_data(1, 8)?;
        }

        Ok(())
    }

    fn get_builder(&self) -> &InBuilder {
        &self.builder
    }

    fn get_event(&self) -> u16 {
        0x0003
    }
}
