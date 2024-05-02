use super::errors::{Error, ErrorKind};

/// Version is an enum that defines the SHDP versions available.
#[derive(PartialEq, Eq, Hash)]
pub enum Version {
    V1,
}

impl Version {
    /// from_u8 is a method that converts a u8 to a Version.
    pub fn from_u8(value: u8) -> Result<Self, Error> {
        match value {
            1 => Ok(Self::V1),
            _ => Err(Error {
                code: 0b1010,
                message: format!("Unknown version: {}", value),
                kind: ErrorKind::UnknownVersion,
            }),
        }
    }

    /// to_u8 is a method that converts a Version to a u8.
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::V1 => 1,
        }
    }
}
