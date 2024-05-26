//!
//! Defines everything for the 0x0001 event.
//!

use std::collections::HashMap;

use bitvec::order::Msb0;

use crate::protocol::{
    client::bits::utils::{FyveImpl, OperatingCode, OperationCode},
    prelude::common::{
        bits::{util::BitReversible, BitDecoder, Frame},
        error::Error,
        event::{EventDecoder, EventEncoder},
    },
};

///
/// Describe a basic HTML content.
/// It can be a text or a child tag.
///
#[derive(Clone, Debug)]
pub enum HtmlContent {
    /// A text element.
    Text(String),
    /// A child tag.
    Child(HtmlTag),
}

impl HtmlContent {
    ///
    /// Get the text of the content.
    /// If the content is a child tag, it will return an empty string.
    ///
    pub fn get_text(&self) -> String {
        match self {
            HtmlContent::Text(text) => text.clone(),
            _ => String::new(),
        }
    }

    ///
    /// Get the child tag of the content.
    /// If the content is a text element, it will return an empty tag.
    ///
    pub fn get_child(&self) -> HtmlTag {
        match self {
            HtmlContent::Child(tag) => tag.clone(),
            _ => HtmlTag {
                name: String::new(),
                attributes: HashMap::new(),
                data: Vec::new(),
            },
        }
    }
}

///
/// Describe a basic HTML tag.
/// It can have a name, attributes, and data.
///
#[derive(Clone, Debug)]
pub struct HtmlTag {
    /// The name of the tag.
    name: String,
    /// The attributes of the tag.
    attributes: HashMap<String, String>,
    /// The data of the tag.
    data: Vec<HtmlContent>,
}

impl HtmlTag {
    ///
    /// Creates a new tag.
    ///
    pub fn new(name: String) -> Self {
        HtmlTag {
            name,
            attributes: HashMap::new(),
            data: Vec::new(),
        }
    }

    ///
    /// Adds an attribute to the tag.
    ///
    pub fn add_attribute(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }

    ///
    /// Adds data to the tag.
    ///
    pub fn add_data(&mut self, data: HtmlContent) {
        self.data.push(data);
    }

    ///
    /// Get the name of the tag.
    ///
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

///
/// Describe a HTML file response.
///
pub struct HtmlFileResponse {
    decoder: BitDecoder<Msb0>,
    /// The name of the file.
    pub name: String,
    /// The parent tag.
    pub parent: HtmlTag,
}

impl HtmlFileResponse {
    ///
    /// Creates a new HTML file response.
    /// It will decode the data and create the tags.
    ///
    /// # Arguments
    /// * `decoder` - The [`BitDecoder<Msb0>`] to decode the response.
    /// * `parent` - The parent tag of the response.
    ///
    /// # Returns
    /// * [HtmlFileResponse] - The created [HtmlFileResponse].
    ///
    /// # Example
    /// ```rust
    /// use shdp::prelude::client::versions::v1::r0x0001::HtmlFileResponse;
    /// use shdp::prelude::common::bits::BitDecoder;
    /// use bitvec::order::Msb0;
    ///
    /// let decoder = BitDecoder::<Msb0>::new(Vec::new());
    /// let response = HtmlFileResponse::new(decoder);
    ///
    /// // These are default values.
    /// assert_eq!(response.name, String::new());
    /// assert_eq!(response.parent.get_name(), String::from(""));
    /// ```
    pub fn new(decoder: BitDecoder<Msb0>) -> Self {
        if cfg!(feature = "debug") {
            println!("[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;21m0x0001\x1b[0m received");
        }

        HtmlFileResponse {
            decoder,
            name: String::new(),
            parent: HtmlTag {
                name: String::new(),
                attributes: HashMap::new(),
                data: Vec::new(),
            },
        }
    }

    fn read_utf8_chain(&mut self, length: u32) -> Result<String, Error> {
        let mut bytes = Vec::<u8>::new();

        for _ in 0..length {
            bytes.push(self.decoder.read_data(8)? as u8);
        }

        println!("{:?}", bytes);

        match String::from_utf8(bytes) {
            Ok(name) => Ok(name),
            Err(_) => {
                return Err(Error {
                    code: 401,
                    message: "Invalid UTF-8".to_string(),
                    kind: crate::protocol::prelude::common::error::ErrorKind::BadRequest,
                });
            }
        }
    }
}

impl EventDecoder<Msb0> for HtmlFileResponse {
    fn decode(
        &mut self,
        frame: Frame<Msb0>,
    ) -> Result<(), crate::protocol::prelude::common::error::Error> {
        let mut bytes = Vec::<u8>::new();
        let mut temp_byte: u8;

        loop {
            temp_byte = self.decoder.read_data(8)? as u8;

            if temp_byte == 0 {
                break;
            }

            bytes.push(temp_byte);
        }

        self.name = match String::from_utf8(bytes) {
            Ok(name) => name,
            Err(_) => {
                return Err(crate::protocol::prelude::common::error::Error {
                    code: 400,
                    message: "Invalid UTF-8".to_string(),
                    kind: crate::protocol::prelude::common::error::ErrorKind::BadRequest,
                });
            }
        };

        let mut is_in_tag = false;
        let mut is_in_attributes = false;
        let mut entered_in_attributes = false;
        let mut entered_in_data = false;
        let mut is_in_data = false;
        let mut text = String::new();
        let mut attribute_name = String::new();
        let mut tag_name = String::new();
        let mut tags_controlled = Vec::<HtmlTag>::new();

        tags_controlled.push(self.parent.clone());

        loop {
            if self.decoder.position >= frame.data_size.into() {
                break;
            }

            let op_code = FyveImpl::get_op(&mut self.decoder)?;

            println!("{:?}", op_code);

            if op_code.kind == OperatingCode::System {
                match op_code.code {
                    Some(OperationCode::StartOfTag) => {
                        is_in_tag = true;
                        is_in_attributes = false;
                        is_in_data = false;
                    }
                    Some(OperationCode::StartOfAttributes) => {
                        is_in_tag = false;
                        is_in_attributes = true;
                        is_in_data = false;
                    }
                    Some(OperationCode::StartOfData) => {
                        is_in_tag = false;
                        is_in_attributes = false;
                        is_in_data = true;
                    }
                    Some(OperationCode::EndOfData) => {
                        is_in_tag = false;
                        is_in_attributes = false;
                        is_in_data = false;
                    }
                    Some(OperationCode::Utf8Chain) => {
                        let text_len = self.decoder.read_data(15)?;
                        text = self.read_utf8_chain(text_len)?;
                    }
                    Some(OperationCode::Unknown) => {
                        return Err(Error {
                            code: 400,
                            message: String::from(format!("Unknown operation code: {:?}", op_code)),
                            kind: crate::protocol::prelude::common::error::ErrorKind::BadRequest,
                        });
                    }
                    None => {
                        return Err(Error {
                            code: 400,
                            message: "Invalid operation code".to_string(),
                            kind: crate::protocol::prelude::common::error::ErrorKind::BadRequest,
                        });
                    }
                }

                if is_in_tag {
                    text = String::new();
                }

                if is_in_attributes && !text.is_empty() {
                    let tag = tags_controlled.get_mut(0).unwrap();
                    tag.add_attribute(attribute_name.clone(), text.clone());
                    attribute_name = String::new();
                    text = String::new();
                } else if is_in_attributes && text.is_empty() && !entered_in_attributes {
                    let tag = HtmlTag {
                        name: tag_name.clone(),
                        attributes: HashMap::new(),
                        data: Vec::new(),
                    };

                    tags_controlled
                        .get_mut(0)
                        .unwrap()
                        .data
                        .push(HtmlContent::Child(tag.clone()));
                    tags_controlled.insert(0, tag);

                    entered_in_attributes = true;
                }

                if is_in_data && !text.is_empty() {
                    tags_controlled
                        .get_mut(0)
                        .unwrap()
                        .data
                        .push(HtmlContent::Text(text.clone()));
                    entered_in_data = true;
                }

                if is_in_data && !entered_in_attributes && !entered_in_data {
                    let tag = HtmlTag {
                        name: tag_name.clone(),
                        attributes: HashMap::new(),
                        data: Vec::new(),
                    };

                    tags_controlled
                        .get_mut(0)
                        .unwrap()
                        .data
                        .push(HtmlContent::Child(tag.clone()));
                    tags_controlled.insert(0, tag);

                    entered_in_data = true;
                } else if !is_in_data {
                    entered_in_data = false;
                }

                if is_in_data && entered_in_attributes {
                    entered_in_attributes = false;
                }

                if !is_in_tag && !is_in_attributes && !is_in_data {
                    tag_name = String::new();
                    attribute_name = String::new();
                    is_in_data = false;

                    if !tags_controlled.is_empty() {
                        tags_controlled.remove(0);
                    }
                }
            }

            if op_code.kind == OperatingCode::Character {
                let char = op_code.get_char()?;
                println!("{:?}", char);

                if is_in_tag {
                    tag_name.push(char);
                }

                if is_in_attributes {
                    attribute_name.push(char);
                }
            }
        }

        self.parent = tags_controlled.get(0).unwrap().clone();

        Ok(())
    }

    fn get_responses(
        &self,
    ) -> Result<Vec<Box<dyn EventEncoder<<Msb0 as BitReversible>::Opposite>>>, Error> {
        Ok(Vec::new())
    }
}
