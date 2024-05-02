#[cfg(feature = "serde")]
use serde_json::Value;

use crate::protocol::errors::Error;

use super::errors::ErrorKind;

pub enum Arg {
    Text(String),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Boolean(bool),
    VecText(Vec<String>),
    OptionText(Option<String>),
    #[cfg(feature = "serde")]
    OptionValue(Option<Value>),
}

impl Arg {
    pub fn from_string(value: &str) -> Self {
        if value.starts_with("0x") {
            if let Ok(value) = u32::from_str_radix(&value[2..], 16) {
                return Arg::U32(value);
            }
        }

        if let Ok(value) = value.parse::<u8>() {
            return Arg::U8(value);
        }

        if let Ok(value) = value.parse::<u16>() {
            return Arg::U16(value);
        }

        if let Ok(value) = value.parse::<u32>() {
            return Arg::U32(value);
        }

        if let Ok(value) = value.parse::<u64>() {
            return Arg::U64(value);
        }

        if value == "true" {
            return Arg::Boolean(true);
        }

        if value == "false" {
            return Arg::Boolean(false);
        }

        Arg::Text(value.to_string())
    }

    pub fn to_string(&self) -> String {
        match self {
            Arg::Text(value) => value.to_string(),
            Arg::U8(value) => value.to_string(),
            Arg::U16(value) => value.to_string(),
            Arg::U32(value) => value.to_string(),
            Arg::U64(value) => value.to_string(),
            Arg::Boolean(value) => value.to_string(),
            Arg::VecText(value) => value.join(", "),
            Arg::OptionText(value) => match value {
                Some(value) => value.to_string(),
                None => "".to_string(),
            },
            #[cfg(feature = "serde")]
            Arg::OptionValue(value) => match value {
                Some(value) => value.to_string(),
                None => "".to_string(),
            },
        }
    }

    pub fn to_u8(&self) -> Result<u8, Error> {
        match self {
            Arg::U8(value) => Ok(*value),
            _ => Err(Error {
                code: 500,
                message: "Expected u8".to_string(),
                kind: ErrorKind::InternalServerError,
            }),
        }
    }

    pub fn to_u16(&self) -> Result<u16, Error> {
        match self {
            Arg::U16(value) => Ok(*value),
            _ => Err(Error {
                code: 500,
                message: "Expected u16".to_string(),
                kind: ErrorKind::InternalServerError,
            }),
        }
    }

    pub fn to_u32(&self) -> Result<u32, Error> {
        match self {
            Arg::U32(value) => Ok(*value),
            _ => Err(Error {
                code: 500,
                message: "Expected u32".to_string(),
                kind: ErrorKind::InternalServerError,
            }),
        }
    }

    pub fn to_u64(&self) -> Result<u64, Error> {
        match self {
            Arg::U64(value) => Ok(*value),
            _ => Err(Error {
                code: 500,
                message: "Expected u64".to_string(),
                kind: ErrorKind::InternalServerError,
            }),
        }
    }

    pub fn to_bool(&self) -> Result<bool, Error> {
        match self {
            Arg::Boolean(value) => Ok(*value),
            _ => Err(Error {
                code: 500,
                message: "Expected boolean".to_string(),
                kind: ErrorKind::InternalServerError,
            }),
        }
    }

    pub fn to_vec_text(&self) -> Result<Vec<String>, Error> {
        match self {
            Arg::VecText(value) => Ok(value.clone()),
            _ => Err(Error {
                code: 500,
                message: "Expected Vec<String>".to_string(),
                kind: ErrorKind::InternalServerError,
            }),
        }
    }

    pub fn to_option_text(&self) -> Result<Option<String>, Error> {
        match self {
            Arg::OptionText(value) => Ok(value.clone()),
            _ => Err(Error {
                code: 500,
                message: "Expected Option<String>".to_string(),
                kind: ErrorKind::InternalServerError,
            }),
        }
    }

    #[cfg(feature = "serde")]
    pub fn to_option_value(&self) -> Result<Option<Value>, Error> {
        match self {
            Arg::OptionValue(value) => Ok(value.clone()),
            _ => Err(Error {
                code: 500,
                message: "Expected Option<Value>".to_string(),
                kind: ErrorKind::InternalServerError,
            }),
        }
    }
}
