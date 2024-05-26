//!
//! Defines everything for the 0x0005 event.
//!

use bitvec::order::Msb0;
use serde_json::Value;

use crate::protocol::prelude::common::{bits::BitEncoder, error::Error, event::EventEncoder};

///
/// Describe an interaction request.
///
pub struct InteractionRequest {
    encoder: BitEncoder<Msb0>,
    request_id: u64,
    function_name: String,
    parent_name: String,
    object_id: Option<i32>,
    params: Option<Value>,
    token: Option<String>,
}

impl InteractionRequest {
    ///
    /// Creates a new [InteractionRequest].
    ///
    /// # Arguments
    /// * `request_id` - The unique request ID.
    /// * `function_name` - The name of the function to call.
    /// * `parent_name` - The name of the table to interact with.
    /// * `object_id` - The ID of the object to interact with.
    /// * `params` - The parameters to pass to the function.
    /// * `token` - The token to authenticate the request.
    ///
    /// # Returns
    /// * [InteractionRequest] - The created [InteractionRequest].
    ///
    /// # Example
    /// ```rust
    /// use shdp::prelude::client::versions::v1::c0x0005::InteractionRequest;
    ///
    /// let request = InteractionRequest::new(
    ///     0,
    ///     "function_name".to_string(),
    ///     "parent_name".to_string(),
    ///     Some(0),
    ///     Some(serde_json::json!({})),
    ///     Some("token".to_string()),
    /// );
    /// ```
    ///
    pub fn new(
        request_id: u64,
        function_name: String,
        parent_name: String,
        object_id: Option<i32>,
        params: Option<Value>,
        token: Option<String>,
    ) -> Self {
        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;21m0x0005\x1b[0m created ({}:{}->{}={:#?}({:#?}, {:#?}))",
                request_id,
                parent_name,
                function_name,
                object_id,
                params,
                token
            );
        }

        InteractionRequest {
            encoder: BitEncoder::<Msb0>::new(),
            request_id,
            function_name,
            parent_name,
            object_id,
            params,
            token,
        }
    }
}

impl EventEncoder<Msb0> for InteractionRequest {
    fn encode(&mut self) -> Result<(), Error> {
        let request_id = self.request_id as u64;
        let upper_id = ((request_id >> 32) & 0xffffffff) as u32;
        let lower_id = (request_id & 0xffffffff) as u32;

        self.encoder.add_data(upper_id, 32)?;
        self.encoder.add_data(lower_id, 32)?;

        self.encoder.add_bytes(self.function_name.as_bytes())?;
        self.encoder.add_data(0, 8)?;
        self.encoder.add_bytes(self.parent_name.as_bytes())?;

        self.encoder.add_data(0, 8)?;
        if self.token.is_some() {
            self.encoder
                .add_bytes(self.token.as_ref().unwrap().to_string().as_bytes())?;
        }

        self.encoder.add_data(0, 8)?;
        if self.object_id.is_some() {
            self.encoder
                .add_bytes(self.object_id.unwrap().to_string().as_bytes())?;
        }

        self.encoder.add_data(0, 8)?;
        if self.params.is_some() {
            self.encoder
                .add_bytes(self.params.as_ref().unwrap().to_string().as_bytes())?;
        }

        Ok(())
    }

    fn get_encoder(&self) -> &BitEncoder<Msb0> {
        &self.encoder
    }

    fn get_event(&self) -> u16 {
        0x0005
    }
}
