use std::{fs::File, io::Read, path::Path};

use crate::protocol::{
    errors::Error,
    server::{bits::builder::InBuilder, event::EventBuilder},
};

pub struct FullFyveResponse {
    builder: InBuilder,
    path: String,
}

impl FullFyveResponse {
    pub fn new(path: String) -> Self {
        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;227mSHDP\x1b[0m] \x1b[38;5;192m0x0004\x1b[0m created ({})",
                path
            );
        }

        FullFyveResponse {
            builder: InBuilder::new(),
            path,
        }
    }
}

impl EventBuilder for FullFyveResponse {
    fn construct(&mut self) -> Result<(), Error> {
        let file_name = Path::new(&self.path)
            .file_name()
            .ok_or(Error {
                code: 400,
                message: format!("Invalid file name: {}", self.path),
                kind: crate::protocol::errors::ErrorKind::BadRequest,
            })?
            .to_str()
            .ok_or(Error {
                code: 400,
                message: format!("Invalid file name: {}", self.path),
                kind: crate::protocol::errors::ErrorKind::BadRequest,
            })?
            .to_string();

        self.builder.add_bytes(file_name.as_bytes())?;
        self.builder.add_data(0, 8)?;

        let mut file = match File::open(&self.path) {
            Ok(file) => file,
            Err(_) => {
                return Err(Error {
                    code: 404,
                    message: format!("File not found: {}", self.path),
                    kind: crate::protocol::errors::ErrorKind::NotFound,
                })
            }
        };

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => (),
            Err(_) => {
                return Err(Error {
                    code: 500,
                    message: format!("File read error: {}", self.path),
                    kind: crate::protocol::errors::ErrorKind::InternalServerError,
                })
            }
        };

        self.builder.add_bytes(content.as_bytes())?;

        Ok(())
    }

    fn get_builder(&self) -> &InBuilder {
        &self.builder
    }

    fn get_event(&self) -> u16 {
        0x0004
    }
}
