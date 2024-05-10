use std::collections::HashMap;

use bitvec::order::Lsb0;

use crate::protocol::prelude::common::{
    bits::BitDecoder,
    error::Error,
    event::EventDecoder,
};

enum OperatingCode {
    System = 0x0,
    Character = 0x1f,
}

impl OperatingCode {
    fn from(fy: &u8) -> OperatingCode {
        match fy {
            0x0 => OperatingCode::System,
            0x1f => OperatingCode::Character,
            _ => OperatingCode::Character,
        }
    }
}

enum OperationCode {
    Utf8Chain = 0x00,
    StartOfTag = 0x10,
    StartOfAttributes = 0x11,
    StartOfData = 0x18,
    EndOfData = 0x19,
    Unknwon = 0xff,
}

enum HtmlContent {
    Text(String),
    Child(HtmlTag),
}

struct HtmlTag {
    name: String,
    attributes: HashMap<String, String>,
    data: Vec<HtmlContent>,
}

impl OperationCode {
    fn from(fy: &u8) -> OperationCode {
        match fy {
            0x00 => OperationCode::Utf8Chain,
            0x10 => OperationCode::StartOfTag,
            0x11 => OperationCode::StartOfAttributes,
            0x18 => OperationCode::StartOfData,
            0x19 => OperationCode::EndOfData,
            _ => OperationCode::Unknwon,
        }
    }
}

struct Operation {
    kind: OperatingCode,
    code: Option<OperationCode>,
    value: Vec<u8>,
}

impl Operation {
    fn from(
        fy: &u8,
        decoder: &mut BitDecoder<Lsb0>
    ) -> Result<Operation, Error> {
        let mut op = OperatingCode::from(fy);
        let mut code: Option<OperationCode> = None;

        let mut value = Vec::<u8>::new();
        value.push(*fy);

        match op {
            OperatingCode::System => {
                let operator = decoder.read_data(5)? as u8;
                code = Some(OperationCode::from(&operator));
                value.push(operator);
            }
            _ => {
                op = OperatingCode::Character;
                if *fy == 0x1f {
                    let mut next_fyve = decoder.read_data(5)? as u8;
                    value.push(next_fyve.clone());

                    while next_fyve == 0x1f {
                        next_fyve = decoder.read_data(5)? as u8;
                        value.push(next_fyve.clone());
                    }
                }
            }
        }

        Ok(Operation {
            kind: op,
            code,
            value,
        })
    }
}

pub struct HtmlFileResponse {
    decoder: BitDecoder<Lsb0>,
    pub name: String,
    pub content: String,
}

impl HtmlFileResponse {
    pub fn new(decoder: BitDecoder<Lsb0>) -> Self {
        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;187mSHDP\x1b[0m] \x1b[38;5;21m0x0001\x1b[0m received"
            );
        }

        HtmlFileResponse {
            decoder,
            name: String::new(),
            content: String::new(),
        }
    }

    fn read_fyve(&mut self) -> Result<u8, Error> {
        Ok(self.decoder.read_data(5)? as u8)
    }
}

impl EventDecoder<Lsb0> for HtmlFileResponse {
    fn decode(
        &mut self
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
                    kind: crate::protocol::prelude::common::error::ErrorKind::BadMapping,
                });
            }
        };

        let mut is_in_tag = false;
        let mut is_in_attributes = false;
        let mut entered_in_attributes = false;
        let mut entered_in_data = false;
        let mut is_in_data = false;
        let mut text: String;
        let mut attribute_name: String;
        let mut tag_name: String;

        Ok(())
    }

    fn get_responses(
        &self
    ) -> Result<
        Vec<
            Box<
                dyn crate::protocol::prelude::common::event::EventEncoder<<Lsb0 as crate::protocol::prelude::common::bits::util::BitReversible>::Opposite>
            >
        >,
        crate::protocol::prelude::common::error::Error
    > {
        Ok(Vec::new())
    }
}
