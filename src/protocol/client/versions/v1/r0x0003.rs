//!
//! Defines everything for the 0x0003 event.
//!

use bitvec::order::Msb0;

use crate::protocol::prelude::common::{
    bits::{ util::BitReversible, BitDecoder, Frame },
    error::Error,
    event::{ EventDecoder, EventEncoder },
};

///
/// Describe an component needs response.
///
pub struct ComponentNeedsResponse {
    decoder: BitDecoder<Msb0>,
    /// The component name.
    pub component_name: String,
    /// The component's title.
    pub title: Option<String>,
    /// The component's filenames.
    pub files: Vec<String>,
}

impl ComponentNeedsResponse {
    ///
    /// Creates a new [ComponentNeedsResponse].
    ///
    /// # Arguments
    /// * `decoder` - The [`BitDecoder<Msb0>`] to decode the request.
    ///
    /// # Returns
    /// * [ComponentNeedsResponse] - The created [ComponentNeedsResponse].
    ///
    /// # Example
    /// ```rust
    /// use shdp::prelude::client::versions::v1::r0x0003::ComponentNeedsResponse;
    /// use shdp::prelude::common::bits::BitDecoder;
    /// use bitvec::order::Msb0;
    ///
    /// let decoder = BitDecoder::<Msb0>::new(Vec::new());
    /// let response = ComponentNeedsResponse::new(decoder);
    ///
    /// // These are default values.
    /// assert_eq!(response.component_name, String::new());
    /// assert_eq!(response.title, None);
    /// assert_eq!(response.files, Vec::<String>::new());
    /// ```
    pub fn new(decoder: BitDecoder<Msb0>) -> Self {
        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;21m0x0003\x1b[0m received"
            );
        }

        ComponentNeedsResponse {
            decoder,
            component_name: String::new(),
            title: None,
            files: Vec::new(),
        }
    }
}

impl EventDecoder<Msb0> for ComponentNeedsResponse {
    fn decode(&mut self, frame: Frame<Msb0>) -> Result<(), Error> {
        // Read bytes till the end.
        let mut bytes = Vec::<u8>::new();

        for _ in 0..frame.data_size / 8 {
            bytes.push(self.decoder.read_data(8)? as u8);
        }

        let data = String::from_utf8(bytes).unwrap();
        let mut parts: Vec<&str> = data.split('\0').collect();
        let component_names: Vec<&str> = parts
            .get(0)
            .unwrap()
            .split('\x01')
            .collect();
        parts.remove(0);

        self.component_name = component_names.get(0).unwrap().to_string();

        match component_names.get(1) {
            Some(title) => {
                self.title = Some(title.to_string());
            }
            None => (),
        }

        for part in parts {
            self.files.push(part.to_string());
        }

        Ok(())
    }

    fn get_responses(
        &self
    ) -> Result<
        Vec<Box<dyn EventEncoder<<Msb0 as BitReversible>::Opposite>>>,
        Error
    > {
        Ok(Vec::new())
    }
}
