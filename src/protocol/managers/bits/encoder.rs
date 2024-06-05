use bitvec::vec::BitVec;

use crate::protocol::{
    errors::{ Error, ErrorKind },
    managers::event::EventEncoder,
    versions::Version,
};

use super::prelude::BitReversible;

///
/// The general encoder structure that permits constructing frames.
///
/// To construct a frame properly, you have to specify the version of the frame.
///
/// The version is a [u8] that represents the version of the frame. See [Version] for more information.
///
/// # Example
/// ```rust
/// use bitvec::order::Lsb0;
///
/// use shdp::prelude::common::bits::FrameEncoder;
///
/// let mut encoder = FrameEncoder::<Lsb0>::new(1).unwrap();
/// ```
///
#[derive(Debug)]
pub struct FrameEncoder<R: BitReversible> {
    encoder: BitEncoder<R>,
    version: Version,
}

impl<R: BitReversible> FrameEncoder<R> {
    /// Create a new FrameEncoder with a specific version.
    /// See [Version] for more information.
    ///
    /// # Arguments
    /// * `version` - The version of the frame.
    ///
    pub fn new(version: u8) -> Result<Self, Error> {
        Ok(Self {
            encoder: BitEncoder::new(),
            version: Version::from_u8(version)?,
        })
    }

    /// Construct a frame with a specific event.
    /// It returns the frame as a [`Vec<u8>`].
    ///
    /// # Arguments
    /// * `frame` - The event encoder to construct the frame.
    ///
    /// # Errors
    /// It can return an [ErrorKind::SizeConstraintViolation] if the data overflows a [u32].
    /// It can return an [ErrorKind::SizeConstraintViolation] if the data is less than 8 bits.
    /// It can return an [ErrorKind::SizeConstraintViolation] if the frame is not well-formed.
    ///
    /// See [EventEncoder] for more information.
    ///
    pub fn encode(
        &mut self,
        mut frame: Box<dyn EventEncoder<R>>
    ) -> Result<Vec<u8>, Error> {
        self.encoder.add_data(self.version.to_u8() as u32, 8)?;
        frame.encode()?;

        let data_size = frame.get_encoder().frame.len();

        if data_size > 1 << 32 {
            return Err(Error {
                code: 0b1000,
                message: "Maximum of 2^32 bits allowed".to_string(),
                kind: ErrorKind::SizeConstraintViolation,
            });
        }

        if data_size < 8 {
            return Err(Error {
                code: 0b1001,
                message: "Minimum of 8 bits allowed".to_string(),
                kind: ErrorKind::SizeConstraintViolation,
            });
        }

        self.encoder.add_data(frame.get_event() as u32, 16)?;
        self.encoder.add_data(data_size as u32, 32)?;

        self.encoder.append_data_from(frame.get_encoder());

        while self.encoder.frame.len() % 8 != 0 {
            self.encoder.add_data(0, 1)?;
        }

        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;227mSHDP\x1b[0m] Sent: In-size {}b / {}B, out-size {}b / {}B",
                data_size,
                (data_size + 8 - (data_size % 8)) / 8,
                self.encoder.frame.len(),
                self.encoder.frame.len() / 8
            );
        }

        Ok(self.encoder.encode())
    }
}

///
/// The bit encoder is a simple structure that permits writing bits to a [BitVec<u8, O: BitOrder>].
///
/// It is used to construct SHDP frames.
///
///
#[derive(Clone, Debug)]
pub struct BitEncoder<R: BitReversible> {
    /// The frame that contains the bits.
    pub frame: BitVec<u8, R>,
}

impl<R: BitReversible> BitEncoder<R> {
    /// Create a new BitEncoder.
    pub fn new() -> Self {
        Self {
            frame: BitVec::<u8, R>::new(),
        }
    }

    ///
    /// Print the frame into the console.
    ///
    #[cfg(feature = "debug")]
    #[allow(dead_code)]
    pub fn print_frame(&self) {
        println!("Frame size: {}", self.frame.len());
        for bit in &self.frame {
            print!("{}", if *bit { 1 } else { 0 });
        }
        println!();
    }

    ///
    /// Adds data to the frame.
    ///
    /// # Arguments
    /// * `data` - The data to add.
    /// * `n` - The number of bits to add the data needs to represented by.
    ///
    /// # Errors
    /// It can return an [ErrorKind::SizeConstraintViolation] if you try to add more than 2^32 bits; [u32::MAX].
    /// It can return an [ErrorKind::SizeConstraintViolation] if you try to add data of more than 32 bits per data.
    ///
    /// ## Error example
    /// ```rust
    /// use bitvec::prelude::Lsb0;
    ///
    /// use shdp::prelude::common::{
    ///     error::{Error, ErrorKind},
    ///     bits::BitEncoder,
    /// };
    ///
    /// let mut encoder = BitEncoder::<Lsb0>::new();
    ///
    /// assert_eq!(encoder.add_data(1, 33).is_err(), true);
    /// ```
    ///
    /// # Example
    /// When everything is good, it returns a mutable reference to the encoder.
    /// ```rust
    /// use bitvec::prelude::Lsb0;
    ///
    /// use shdp::prelude::common::bits::BitEncoder;
    ///
    /// let mut encoder = BitEncoder::<Lsb0>::new();
    ///
    /// assert_eq!(encoder.add_data(1, 1).is_ok(), true);
    /// ```
    pub fn add_data(&mut self, data: u32, n: u8) -> Result<&mut Self, Error> {
        if self.frame.len() + (n as usize) > 1 << 32 {
            return Err(Error {
                code: 0b1000,
                message: "Maximum of 2^32 bits allowed".to_string(),
                kind: ErrorKind::SizeConstraintViolation,
            });
        }

        if n > 32 {
            return Err(Error {
                code: 0b1000,
                message: "Data of more than 32 bits long are not allowed".to_string(),
                kind: ErrorKind::SizeConstraintViolation,
            });
        }

        for i in (0..n).rev() {
            let bit = ((data >> i) & 1) == 1;
            self.frame.push(bit);
        }

        Ok(self)
    }

    ///
    /// Adds bytes to the frame.
    /// It is a simple wrapper around [BitEncoder::add_data].
    /// It adds each byte of the slice to the frame.
    ///
    /// # Arguments
    /// * `bytes` - The bytes to add.
    ///
    /// # Errors
    /// See [BitEncoder::add_data] for more information.
    ///
    /// # Example
    /// ```rust
    /// use bitvec::order::Lsb0;
    ///
    /// use shdp::prelude::common::bits::BitEncoder;
    ///
    /// let mut encoder = BitEncoder::<Lsb0>::new();
    /// encoder.add_bytes("test".as_bytes()).unwrap();
    ///
    /// assert_eq!(encoder.frame.len(), 32);
    /// ```
    pub fn add_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self, Error> {
        for &byte in bytes {
            self.add_data(byte as u32, 8)?;
        }

        Ok(self)
    }

    ///
    /// Adds a bitvec to the frame.
    ///
    /// # Arguments
    /// * `bitvec` - The bitvec to add.
    ///
    /// # Errors
    /// See [BitEncoder::add_data] for more information.
    ///
    /// # Example
    /// ```rust
    /// use bitvec::order::Lsb0;
    /// use bitvec::bitvec;
    ///
    /// use shdp::prelude::common::bits::BitEncoder;
    ///
    /// let mut encoder = BitEncoder::<Lsb0>::new();
    /// let bitvec = bitvec![u8, Lsb0; 1, 0, 1, 0, 1, 0, 1, 0];
    /// encoder.add_bitvec(&bitvec).unwrap();
    ///
    /// assert_eq!(encoder.frame.len(), 8);
    /// ```
    pub fn add_bitvec(
        &mut self,
        bitvec: &BitVec<u8, R>
    ) -> Result<&mut Self, Error> {
        for bit in bitvec {
            self.frame.push(*bit);
        }

        Ok(self)
    }

    ///
    /// Appends data from another encoder.
    ///
    /// # Arguments
    /// * `encoder` - The encoder to append data from.
    ///
    /// # Example
    /// ```rust
    /// use bitvec::order::Lsb0;
    ///
    /// use shdp::prelude::common::bits::BitEncoder;
    ///
    /// let mut encoder_1 = BitEncoder::<Lsb0>::new();
    /// let mut encoder_2 = BitEncoder::<Lsb0>::new();
    /// encoder_2.add_data(1, 8).unwrap();
    ///
    /// encoder_1.append_data_from(&encoder_2);
    ///
    /// assert_eq!(encoder_1.frame.len(), 8);
    /// ```
    pub fn append_data_from(&mut self, encoder: &BitEncoder<R>) {
        self.frame.append(&mut encoder.frame.clone());
    }

    fn reverse_bits_in_bytes(&self, input: &[u8]) -> Vec<u8> {
        input
            .iter()
            .map(|&byte| byte.reverse_bits())
            .collect()
    }

    ///
    /// Encodes the frame.
    ///
    /// It returns the frame as a [`Vec<u8>`].
    ///
    /// # Example
    /// ```rust
    /// use bitvec::order::Lsb0;
    ///
    /// use shdp::prelude::common::bits::BitEncoder;
    ///
    /// let mut encoder = BitEncoder::<Lsb0>::new();
    /// encoder.add_data(1, 8).unwrap();
    /// encoder.add_data(0, 16).unwrap();
    /// encoder.add_data(32, 32).unwrap();
    ///
    /// let frame = encoder.encode();
    ///
    /// assert_eq!(frame.len(), 7);
    /// ```
    pub fn encode(&self) -> Vec<u8> {
        let vec = self.frame.clone().into_vec();
        self.reverse_bits_in_bytes(&vec)
    }
}
