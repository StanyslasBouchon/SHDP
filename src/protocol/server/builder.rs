use crate::protocol::{
    errors::{Error, ErrorKind},
    versions::Version,
};

use super::{bits::builder::InBuilder, event::EventBuilder};

/// OutBuilder is a struct that defines the methods that a SHDP builder must implement.
///
/// It used to build any SHDP event as an SHDP frame to be sent.
pub struct OutBuilder {
    builder: InBuilder,
    version: Version,
}

impl OutBuilder {
    pub fn new(version: u8) -> Result<Self, Error> {
        Ok(Self {
            builder: InBuilder::new(),
            version: Version::from_u8(version)?,
        })
    }

    pub fn construct(&mut self, mut shdp_frame: Box<dyn EventBuilder>) -> Result<Vec<u8>, Error> {
        self.builder.add_data(self.version.to_u8() as u32, 8)?;
        shdp_frame.construct()?;

        let data_size = shdp_frame.get_builder().frame.len();

        if data_size > (1 << 32) {
            return Err(Error {
                code: 0b1000,
                message: "Maximum of 2^32 bits allowed".to_string(),
                kind: ErrorKind::SizeConstraintViolation,
            });
        }

        if data_size < 8 {
            return Err(Error {
                code: 0b1001,
                message: "Minimum of 8 bits allowed".to_string(),
                kind: ErrorKind::SizeConstraintViolation,
            });
        }

        self.builder.add_data(shdp_frame.get_event() as u32, 16)?;
        self.builder.add_data(data_size as u32, 32)?;

        self.builder.append_data_from(shdp_frame.get_builder());

        while self.builder.frame.len() % 8 != 0 {
            self.builder.add_data(0, 1)?;
        }

        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;227mSHDP\x1b[0m] Sent: In-size {}b / {}B, out-size {}b / {}B",
                data_size,
                (data_size + 8 - (data_size % 8)) / 8,
                self.builder.frame.len(),
                self.builder.frame.len() / 8
            );
        }

        Ok(self.builder.build())
    }
}
