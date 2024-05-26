#[cfg(feature = "serde")]
use serde_json::Value;

use crate::protocol::errors::Error;

use super::errors::ErrorKind;

///
/// This enum represents the different types of arguments that can be passed over the SHDP protocol.
///
pub enum Arg {
    /// A text argument.
    Text(String),
    /// An unsigned 8-bit integer argument.
    U8(u8),
    /// An unsigned 16-bit integer argument.
    U16(u16),
    /// An unsigned 32-bit integer argument.
    U32(u32),
    /// A boolean argument.
    Boolean(bool),
    /// A vector of text arguments.
    VecText(Vec<String>),
    /// An optional text argument.
    OptionText(Option<String>),
    ///
    /// An optional value argument.
    ///
    /// This is only available when the `serde` feature is enabled.
    ///
    #[cfg(feature = "serde")]
    OptionValue(Option<Value>),
}

impl Arg {
    ///
    /// Creates an argument from a string.
    ///
    /// The string if converted to the appropriate type, first. Then the argument is created.
    ///
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

        if value == "true" {
            return Arg::Boolean(true);
        }

        if value == "false" {
            return Arg::Boolean(false);
        }

        Arg::Text(value.to_string())
    }

    ///
    /// Converts the argument to a string.
    ///
    pub fn to_string(&self) -> String {
        match self {
            Arg::Text(value) => value.to_string(),
            Arg::U8(value) => value.to_string(),
            Arg::U16(value) => value.to_string(),
            Arg::U32(value) => value.to_string(),
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

    ///
    /// Converts the argument to an unsigned 8-bit integer.
    ///
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

    ///
    /// Converts the argument to an unsigned 16-bit integer.
    ///
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

    ///
    /// Converts the argument to an unsigned 32-bit integer.
    ///
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

    ///
    /// Converts the argument to a boolean.
    ///
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

    ///
    /// Converts the argument to a vector of text.
    ///
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

    ///
    /// Converts the argument to an optional text.
    ///
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

    ///
    /// Converts the argument to an optional value.
    ///
    /// This is only available when the `serde` feature is enabled.
    ///
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
