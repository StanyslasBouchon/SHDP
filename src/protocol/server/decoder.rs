use bitvec::{prelude::Msb0, vec::BitVec};

use crate::protocol::errors::Error;

use super::bits::decoder::InDecoder;

pub struct OutDecoder {
    pub version: u8,
    pub data_size: u16,
    pub event: u16,
    pub data: Box<BitVec<u8, Msb0>>,
}

pub struct Decoder {
    decoder: InDecoder,
}

impl Decoder {
    pub fn new(decoder: InDecoder) -> Self {
        Self { decoder }
    }

    pub fn parse(&mut self) -> Result<OutDecoder, Error> {
        let version = self.decoder.read_data(8)? as u8;
        let event = self.decoder.read_data(16)? as u16;
        let data_size = self.decoder.read_data(32)? as u16;

        let data = Box::new(self.decoder.read_vec(56, 56 + (data_size as usize))?);

        Ok(OutDecoder {
            version,
            data_size,
            event,
            data,
        })
    }
}
