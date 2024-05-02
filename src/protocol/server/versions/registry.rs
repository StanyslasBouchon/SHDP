use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::protocol::{
    args::Arg,
    server::{bits::decoder::InDecoder, event::EventDecoder},
};

use super::v1::{r0x0000::ComponentNeedsRequest, r0x0005::InteractionRequest};

pub struct EventRegistry {
    events: HashMap<(u8, u16), fn(InDecoder) -> Box<dyn EventDecoder>>,
    listeners: HashMap<(u8, u16), fn(Box<dyn EventDecoder>) -> Box<[Arg]>>,
}

impl EventRegistry {
    fn new() -> Self {
        let mut registry = EventRegistry {
            events: HashMap::new(),
            listeners: HashMap::new(),
        };

        registry.add_event(1, 0x0000, |decoder| {
            Box::new(ComponentNeedsRequest::new(decoder))
        });

        registry.add_event(1, 0x0005, |decoder| {
            Box::new(InteractionRequest::new(decoder))
        });

        registry
    }

    pub fn get_event(
        &self,
        version: u8,
        event_code: u16,
    ) -> Option<&fn(InDecoder) -> Box<dyn EventDecoder>> {
        self.events.get(&(version, event_code))
    }

    pub fn get_listener(
        &self,
        version: u8,
        event_code: u16,
    ) -> Option<&fn(Box<dyn EventDecoder>) -> Box<[Arg]>> {
        self.listeners.get(&(version, event_code))
    }

    pub fn add_event(
        &mut self,
        version: u8,
        event_code: u16,
        event: fn(InDecoder) -> Box<dyn EventDecoder>,
    ) {
        self.events.insert((version, event_code), event);
    }

    #[allow(dead_code)]
    pub fn add_listener(
        &mut self,
        version: u8,
        event_code: u16,
        listener: fn(Box<dyn EventDecoder>) -> Box<[Arg]>,
    ) {
        self.listeners.insert((version, event_code), listener);
    }
}

lazy_static! {
    pub static ref EVENT_REGISTRY: EventRegistry = EventRegistry::new();
}
