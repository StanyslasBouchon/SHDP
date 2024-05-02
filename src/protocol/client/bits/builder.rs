use bitvec::{prelude::Msb0, vec::BitVec};

use crate::protocol::errors::{Error, ErrorKind};

#[derive(Clone)]
pub struct InBuilder {
    pub frame: BitVec<u8, Msb0>,
}

impl InBuilder {
    pub fn new() -> Self {
        Self {
            frame: BitVec::<u8, Msb0>::new(),
        }
    }

    #[cfg(feature = "debug")]
    pub fn print_frame(&self) {
        println!("Frame size: {}", self.frame.len());
        for bit in &self.frame {
            print!("{}", if *bit { 1 } else { 0 });
        }
        println!();
    }

    pub fn add_data(&mut self, data: u32, bits: u8) -> Result<&mut Self, Error> {
        if bits > 32 {
            return Err(Error {
                code: 0b1000,
                message: "Maximum of 2^32 bits allowed".to_string(),
                kind: ErrorKind::SizeConstraintViolation,
            });
        }

        for i in (0..bits).rev() {
            let bit = ((data >> i) & 1) == 1;
            self.frame.push(bit);
        }

        Ok(self)
    }

    pub fn add_bitvec(&mut self, mut data: BitVec<u32, Msb0>) -> &mut Self {
        self.frame.append(&mut data);
        self
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self, Error> {
        for &byte in bytes {
            self.add_data(byte as u32, 8)?;
        }

        Ok(self)
    }

    pub fn append_data_from(&mut self, builder: &InBuilder) {
        self.frame.append(&mut builder.frame.clone());
    }

    fn reverse_bits_in_bytes(&self, input: &[u8]) -> Vec<u8> {
        input.iter().map(|&byte| byte.reverse_bits()).collect()
    }

    pub fn build(&self) -> Vec<u8> {
        let vec = self.frame.clone().into_vec();
        self.reverse_bits_in_bytes(&vec)
    }
}
