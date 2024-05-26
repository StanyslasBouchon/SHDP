//!
//! Defines everything for the 0x0000 event.
//!

use bitvec::order::Lsb0;

use crate::protocol::prelude::common::{bits::BitEncoder, error::Error, event::EventEncoder};

///
/// Describe an component needs request.
///
pub struct ComponentNeedsRequest {
    encoder: BitEncoder<Lsb0>,
    component_name: String,
}

impl ComponentNeedsRequest {
    ///
    /// Creates a new [ComponentNeedsRequest].
    ///
    /// # Arguments
    /// * `component_name` - The component name.
    ///
    /// # Returns
    /// * [ComponentNeedsRequest] - The created [ComponentNeedsRequest].
    ///
    /// # Example
    /// ```rust
    /// use shdp::prelude::client::versions::v1::c0x0000::ComponentNeedsRequest;
    ///
    /// let request = ComponentNeedsRequest::new("component_name".to_string());
    /// ```
    ///
    pub fn new(component_name: String) -> Self {
        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;21m0x0000\x1b[0m created ({})",
                component_name
            );
        }

        ComponentNeedsRequest {
            encoder: BitEncoder::<Lsb0>::new(),
            component_name,
        }
    }
}

impl EventEncoder<Lsb0> for ComponentNeedsRequest {
    fn encode(&mut self) -> Result<(), Error> {
        self.encoder.add_bytes(self.component_name.as_bytes())?;

        Ok(())
    }

    fn get_encoder(&self) -> &BitEncoder<Lsb0> {
        &self.encoder
    }

    fn get_event(&self) -> u16 {
        0x0000
    }
}
