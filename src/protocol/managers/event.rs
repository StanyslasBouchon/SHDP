#[allow(unused_imports)]
use bitvec::order::BitOrder;

#[allow(unused_imports)]
use crate::protocol::{
    errors::{Error, ErrorKind},
    managers::bits::encoder::BitEncoder,
};

use super::bits::prelude::BitReversible;

///
/// The event encoder structure that permits constructing events over frames.
///
/// To construct an event properly, you have to create a new struct that implements this trait.
///
/// # Example
/// ```rust
/// use bitvec::order::Lsb0;
///
/// use shdp::prelude::common::{event::EventEncoder, bits::BitEncoder, error::Error};
///
/// pub struct MyEvent {
///     encoder: BitEncoder<Lsb0>,
///     data: u32,
/// }
///
/// impl MyEvent {
///     pub fn new(data: u32) -> Self {
///         MyEvent {
///             encoder: BitEncoder::<Lsb0>::new(),
///             data,
///         }
///     }
/// }
///
/// impl EventEncoder<Lsb0> for MyEvent {
///     fn encode(&mut self) -> Result<(), Error> {
///         self.encoder.add_data(self.data, 32)?;
///         Ok(())
///    }
///
///     fn get_encoder(&self) -> &BitEncoder<Lsb0> {
///         &self.encoder
///     }
///
///     fn get_event(&self) -> u16 {
///         0x0001
///     }
/// }
/// ```
///
pub trait EventEncoder<O: BitReversible> {
    ///
    /// Encode every data into the encoder.
    ///
    /// # Errors
    /// It can return an [ErrorKind::SizeConstraintViolation] if the data overflows a [u32].<br>
    /// It can return an [ErrorKind::SizeConstraintViolation] if the data is less than 8 bits.<br>
    /// It can return an [ErrorKind::SizeConstraintViolation] if the frame is not well-formed.
    ///
    /// It can return any other error based on user implementation.
    ///
    fn encode(&mut self) -> Result<(), Error>;

    ///
    /// Get the encoder.
    ///
    fn get_encoder(&self) -> &BitEncoder<O>;

    ///
    /// Get the event code.
    /// It returns a [u16] that represents the event code.
    ///
    /// !! The event code is a __unique__ identifier and __should not__ be repeated.
    ///
    fn get_event(&self) -> u16;
}

///
/// The event decoder structure that permits decoding events from frames.
///
/// To decode an event properly, you have to create a new struct that implements this trait.
///
/// # Example
/// ```rust
/// use bitvec::order::{Msb0, Lsb0};
///
/// use shdp::prelude::common::{
///     event::{EventDecoder, EventEncoder},
///     bits::BitDecoder,
///     error::Error
/// };
///
/// pub struct MyEvent {
///     decoder: BitDecoder<Msb0>,
///     data: u32,
/// }
///
/// impl MyEvent {
///     pub fn new(decoder: BitDecoder<Msb0>) -> Self {
///         MyEvent {
///             decoder,
///             data: 0,
///         }
///     }
/// }
///
/// impl EventDecoder<Msb0> for MyEvent {
///     fn decode(&mut self) -> Result<(), Error> {
///         self.data = self.decoder.read_data(32)?;
///         Ok(())
///     }
///     
///     fn get_responses(&self) -> Result<Vec<Box<dyn EventEncoder<Lsb0>>>, Error> {
///         Ok(vec![])
///     }
/// }
/// ```
pub trait EventDecoder<R: BitReversible> {
    ///
    /// Decode the frame into the event.
    ///
    /// # Errors
    /// It can return an [ErrorKind::SizeConstraintViolation] if the data overflows a [u32].<br>
    /// It can return an [ErrorKind::SizeConstraintViolation] if the data is less than 8 bits.<br>
    /// It can return an [ErrorKind::SizeConstraintViolation] if the frame is not well-formed.
    ///
    /// It can return any other error based on user implementation.
    ///
    fn decode(&mut self) -> Result<(), Error>;

    ///
    /// Get the responses from the event.
    ///
    /// It returns a [Vec] of [Box]ed [EventEncoder<R: BitOrder>], where the [BitOrder] __should__ be the opposite of the current event's [BitOrder], or [BitReversible::Opposite].
    ///
    /// The responses are the frames that the event will send back to the client.
    ///
    /// # Errors
    /// It can return any error based on user implementation.
    ///
    fn get_responses(&self) -> Result<Vec<Box<dyn EventEncoder<R::Opposite>>>, Error>;
}
