use bitvec::{prelude::Msb0, vec::BitVec};

use crate::protocol::errors::{Error, ErrorKind};

#[derive(Clone)]
pub struct InDecoder {
    pub frame: BitVec<u8, Msb0>,
    position: usize,
}

impl InDecoder {
    pub fn new(frame: Vec<u8>) -> Self {
        Self {
            frame: BitVec::<u8, Msb0>::from_slice(&frame),
            position: 0,
        }
    }

    pub fn read_data(&mut self, bits: u8) -> Result<u32, Error> {
        if self.frame.len() < 2u32.pow((self.position + bits as usize) as u32) as usize {
            return Err(Error {
                code: 0b1000,
                message: "Maximum of 2^32 bits allowed".to_string(),
                kind: ErrorKind::SizeConstraintViolation,
            });
        }

        let mut data = 0;

        for _ in 0..bits {
            let bit = self.frame.get(self.position).map(|b| *b).unwrap_or(false);
            data = (data << 1) | (bit as u32);
            self.position += 1;
        }

        Ok(data)
    }

    pub fn read_vec(&self, from: usize, bits: usize) -> Result<BitVec<u8, Msb0>, Error> {
        if from >= self.frame.len() {
            return Err(Error {
                code: 0b1100,
                message: "Index out of range".to_string(),
                kind: ErrorKind::SizeConstraintViolation,
            });
        }

        Ok(self.frame[from..bits].to_bitvec())
    }

    pub fn get_position(&self) -> usize {
        self.position
    }
}
