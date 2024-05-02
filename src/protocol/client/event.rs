use std::any::Any;

use crate::protocol::errors::Error;

use super::bits::builder::InBuilder;

/// SHDPEventBuilder is a trait that defines the methods that a SHDP event builder must implement.
/// The SHDPEventBuilder trait is used to build SHDP events.
pub trait EventBuilder {
    fn construct(&mut self) -> Result<(), Error>;
    fn get_builder(&self) -> &InBuilder;
    fn set_builder(&mut self, builder: &InBuilder);
    fn get_event(&self) -> u16;
    fn as_any(&self) -> &dyn Any;
}

/// SHDPEventDecoder is a trait that defines the methods that a SHDP event decoder must implement.
/// The SHDPEventDecoder trait is used to decode SHDP events.
pub trait EventDecoder {
    fn parse(&mut self);
    fn get_responses(&self) -> Vec<Box<dyn EventBuilder>>;
}
