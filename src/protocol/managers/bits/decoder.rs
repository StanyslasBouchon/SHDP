use bitvec::vec::BitVec;

use crate::protocol::errors::{ Error, ErrorKind };

#[allow(unused_imports)]
#[cfg(any(feature = "tcp-server", feature = "ws-server"))]
use crate::protocol::server::versions::v1::r0x0000::ComponentNeedsRequest;

use super::prelude::BitReversible;

///
/// The bit decoder is a simple structure that permits reading bits from a [BitVec<u8, O: BitOrder>].
///
#[derive(Clone, Debug)]
pub struct BitDecoder<R: BitReversible> {
    /// The frame to read data from.
    pub frame: BitVec<u8, R>,
    pub position: usize,
}

impl<R: BitReversible> BitDecoder<R> {
    ///
    /// Create a new BitDecoder from a [`Vec<u8>`].
    ///
    /// # Arguments
    /// * `frame` - The frame to read data from.
    ///
    pub fn new(frame: Vec<u8>) -> Self {
        Self {
            frame: BitVec::<u8, R>::from_slice(&frame),
            position: 0,
        }
    }

    ///
    /// Convert the BitVec found from the last position to `n`, where `n` represents the number of bits to read.
    /// It returns the data as a [u32], which is the maximum size of the data.
    ///
    /// # Arguments
    /// * `n` - The number of bits to read.
    ///
    /// # Errors
    /// It can return an [ErrorKind::SizeConstraintViolation] if the data is out of bound.
    ///
    /// ## Error example
    /// ```rust
    /// use bitvec::prelude::Msb0;
    ///
    /// use shdp::prelude::common::{
    ///     error::{Error, ErrorKind},
    ///     bits::BitDecoder,
    /// };
    ///
    /// let mut decoder = BitDecoder::<Msb0>::new(vec![0b0000_0001]);
    ///
    /// assert_eq!(decoder.read_data(16).is_err(), true);
    /// ```
    ///
    /// # Example
    /// When everything is good, it returns the data as a [u32].
    ///
    /// ```rust
    /// use bitvec::prelude::Msb0;
    ///
    /// use shdp::prelude::common::bits::BitDecoder;
    ///
    /// let mut decoder = BitDecoder::<Msb0>::new(vec![0b0000_0001]);
    ///
    /// assert_eq!(decoder.read_data(8).unwrap(), 1 as u32);
    /// ```
    ///
    pub fn read_data(&mut self, n: u8) -> Result<u32, Error> {
        if self.position + (n as usize) > self.frame.len() {
            return Err(Error {
                code: 0b1000,
                message: "Out of bound".to_string(),
                kind: ErrorKind::SizeConstraintViolation,
            });
        }

        let mut data = 0;

        for _ in 0..n {
            let bit = self.frame
                .get(self.position)
                .map(|b| *b)
                .unwrap_or(false);
            data = (data << 1) | (bit as u32);
            self.position += 1;
        }

        Ok(data)
    }

    ///
    /// Read a vector of bits from the frame.
    /// It returns a [BitVec] of the data.
    ///
    /// # Arguments
    /// * `from` - The starting index of the data.
    /// * `to` - The ending index of the data.
    ///
    /// # Errors
    /// It can return an [ErrorKind::SizeConstraintViolation] if the data is out of bound.
    ///
    /// ## Error example
    /// ```rust
    /// use bitvec::prelude::Msb0;
    ///
    /// use shdp::prelude::common::{
    ///     error::{Error, ErrorKind},
    ///     bits::BitDecoder,
    /// };
    ///
    /// let mut decoder = BitDecoder::<Msb0>::new(vec![0b0000_0001]);
    ///
    /// assert_eq!(decoder.read_vec(8, 16).is_err(), true);
    /// ```
    ///
    /// # Example
    /// When everything is good, it returns the data as a [BitVec<u8, O: BitOrder>].
    ///
    /// ```rust
    /// use bitvec::prelude::Msb0;
    /// use bitvec::vec::BitVec;
    ///
    /// use shdp::prelude::common::bits::BitDecoder;
    ///
    /// let mut decoder = BitDecoder::<Msb0>::new(vec![0b0000_0001, 0b0100_0000, 0b0000_0000]);
    ///
    /// assert_eq!(
    ///     decoder.read_vec(8, 24).unwrap(),
    ///     BitVec::<u8, Msb0>::from_slice(&[0b0100_0000, 0b0000_0000])
    /// );
    /// ```
    ///
    pub fn read_vec(
        &self,
        from: usize,
        to: usize
    ) -> Result<BitVec<u8, R>, Error> {
        if from >= self.frame.len() {
            return Err(Error {
                code: 0b1100,
                message: "out of bound".to_string(),
                kind: ErrorKind::SizeConstraintViolation,
            });
        }

        Ok(self.frame[from..to].to_bitvec())
    }
}

/// The SHDP frame structure.
///
/// It contains the version, the data size, the event, and the data.
///
/// It represents every frame received or sent by the SHDP protocol.
#[derive(Debug)]
pub struct Frame<R: BitReversible> {
    pub version: u8,
    #[allow(dead_code)]
    pub data_size: u16,
    pub event: u16,
    #[allow(dead_code)]
    pub data: Box<BitVec<u8, R>>,
}

///
/// The general decoder permits decoding SHDP frame into their basic structure.
///
///
/// Each SHDP frame is composed of:
/// - `8` bits for the <u>version</u>
/// - `16` bits for the <u>event</u>
/// - `32` bits for the <u>data size</u>
///
/// _These are unsigned integers._
///
///
/// These data are called the header.
/// The data size is the size of the data that follows the header in bit-count, not byte-count.
/// This is only because the data could be compressed or composed of fyve data or any other bit structure.
///
///
/// The basic SHDP frame looks like this (bits):
/// <pre>
/// <span style="color: #ababab">0x0000</span> <span style="color: #6996ff">0000 0001</span> <span style="color: #2ef290">0000 0000 0000 0000</span> <span style="color: #e8db20">0000 0000</span>
/// <span style="color: #ababab">0x0001</span> <span style="color: #e8db20">0000 0000 0000 0000 0010 0000</span> <span style="color: #ababab">0111 0100</span>
/// <span style="color: #ababab">0x0002 0110 0101 0110 0101 0111 0100</span>
/// </pre>
///
/// Or in hex:
/// <pre>
/// <span style="color: #ababab">0x0000</span> <span style="color: #6996ff">01</span> <span style="color: #2ef290">00 00</span> <span style="color: #e8db20">00 00 00 20</span> <span style="color: #ababab">74 65 73 74</span>
/// </pre>
///
/// As you can see, the version is colored in blue, the event in green, and the data size in yellow.
/// Here are their value:
/// - version: `1`
/// - event: `0` (which corresponds to the [ComponentNeedsRequest] event)
/// - data size: `32`
///
///
/// # Example
/// ```rust
/// use bitvec::prelude::{Msb0, Lsb0};
///
/// use shdp::prelude::common::bits::{BitDecoder, FrameDecoder, Frame, BitEncoder};
///
/// // Creating the encoder is only for example purposes.
/// let mut encoder = BitEncoder::<Lsb0>::new();
/// encoder.add_data(1, 8).unwrap();
/// encoder.add_data(0, 16).unwrap();
/// encoder.add_data(32, 32).unwrap();
/// encoder.add_bytes("test".as_bytes()).unwrap();
///
/// let received_data: Vec<u8> = encoder.encode();
///
/// // First, we create the decoder from the received data.
/// let decoder = BitDecoder::<Msb0>::new(received_data);
///
/// // Then, we decode the data into an FrameDecoder object.
/// let data: Frame<Msb0> = FrameDecoder::<Msb0>::new(decoder.clone())
///     .decode()
///     .unwrap();
///
/// assert_eq!(data.version, 1);
/// assert_eq!(data.event, 0);
/// assert_eq!(data.data_size, 32);
/// ```
///
#[derive(Debug)]
pub struct FrameDecoder<R: BitReversible> {
    decoder: BitDecoder<R>,
}

impl<R: BitReversible> FrameDecoder<R> {
    ///
    /// Create a new FrameDecoder from a BitDecoder.
    ///
    /// # Arguments
    /// * `decoder` - The BitDecoder to decode the frame from.
    ///
    pub fn new(decoder: BitDecoder<R>) -> Self {
        Self { decoder }
    }

    ///
    /// Decode the SHDP frame into a [Frame] structure.
    ///
    /// It returns the header of the frame, and a [BitVec] of the data.
    ///
    /// # Errors
    ///
    /// It returns an [ErrorKind::SizeConstraintViolation] if any data header is out of bound, or if there is no data (data_size = 0).
    /// It could also return any other error from [ErrorKind] from the [BitDecoder::read_data] or [BitDecoder::read_vec] methods.
    ///
    /// # Error example
    /// ```rust
    /// use std::any::Any;
    ///
    /// use bitvec::prelude::{Msb0, Lsb0};
    ///
    /// use shdp::prelude::common::{
    ///     error::{ErrorKind, Error},
    ///     bits::{BitDecoder, FrameDecoder, BitEncoder},
    /// };
    ///
    /// // Creating the encoder is only for example purposes.
    /// let mut encoder = BitEncoder::<Lsb0>::new();
    /// encoder.add_data(1, 2).unwrap();
    /// encoder.add_data(0, 7).unwrap();
    /// encoder.add_data(2, 3).unwrap();
    /// encoder.add_bytes("test".as_bytes()).unwrap();
    ///
    /// let decoder = BitDecoder::<Msb0>::new(encoder.encode());
    ///
    /// assert_eq!(
    ///     FrameDecoder::<Msb0>::new(decoder.clone()).decode().is_err(),
    ///     true
    /// );
    ///
    /// let err: Error = FrameDecoder::<Msb0>::new(decoder.clone())
    ///     .decode()
    ///     .unwrap_err();
    ///
    /// assert_eq!(err.code, 0b1000); // 8
    /// assert_eq!(
    ///     err.kind.type_id(),
    ///     ErrorKind::SizeConstraintViolation.type_id()
    /// );
    ///```
    ///
    ///
    pub fn decode(&mut self) -> Result<Frame<R>, Error> {
        let version = self.decoder.read_data(8)? as u8;
        let event = self.decoder.read_data(16)? as u16;
        let data_size = self.decoder.read_data(32)? as u16;

        let data = Box::new(
            self.decoder.read_vec(56, 56 + (data_size as usize))?
        );

        Ok(Frame {
            version,
            data_size,
            event,
            data,
        })
    }
}
