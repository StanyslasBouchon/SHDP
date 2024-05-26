use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};

use bitvec::order::{Lsb0, Msb0};
use ctor::ctor;
use lazy_static::lazy_static;

#[cfg(any(feature = "tcp-client", feature = "ws-client"))]
use crate::client::prelude::versions::v1::r0x0001::HtmlFileResponse;
#[cfg(any(feature = "tcp-client", feature = "ws-client"))]
use crate::client::prelude::versions::v1::r0x0002::ErrorResponse;
#[cfg(any(feature = "tcp-client", feature = "ws-client"))]
use crate::client::prelude::versions::v1::r0x0003::ComponentNeedsResponse;
#[cfg(any(feature = "tcp-client", feature = "ws-client"))]
use crate::client::prelude::versions::v1::r0x0004::FullFyveResponse;
#[cfg(all(any(feature = "tcp-client", feature = "ws-client"), feature = "serde"))]
use crate::client::prelude::versions::v1::r0x0006::InteractionResponse;
use crate::protocol::managers::bits::prelude::BitReversible;
use crate::protocol::managers::event::EventDecoder;
#[cfg(any(feature = "tcp-server", feature = "ws-server"))]
use crate::protocol::server::versions::v1::r0x0000::ComponentNeedsRequest;
#[cfg(all(feature = "serde", any(feature = "tcp-server", feature = "ws-server")))]
use crate::protocol::server::versions::v1::r0x0005::InteractionRequest;
use crate::protocol::{args::Arg, managers::bits::decoder::BitDecoder};

///
/// The event id is a tuple that represents the version and the event code: (version, event_code).
///
pub type EventId = (u8, u16);

///
/// The event function is a function that will be called when the event is received.<br>
/// It allows parsing the corresponding event to its structure.
///
pub type EventFn<R> = fn(bit_decoder: BitDecoder<R>) -> Box<dyn EventDecoder<R>>;

///
/// The listener function is a function that will be called when the event is received.
/// It allows getting the resources needed to answer the event.
///
pub type ListenerFn<R> = fn(event_decoder: Box<dyn EventDecoder<R>>) -> Box<[Arg]>;

///
/// The event registry is a structure that permits storing events and listeners.
///
/// It is used to store events and listeners that will be by each the server and the client.
///
/// To store event or listener, you have to use both [static@EVENT_REGISTRY_MSB] and [static@EVENT_REGISTRY_LSB], whether you are a server or a client.
/// * `Msb0` - For server requests or client responses.
/// * `Lsb0` - For server responses or client requests.
///
/// # Example
///
/// ```rust
/// use std::sync::{Arc, Mutex};
///
/// use bitvec::order::{Msb0, Lsb0};
///
/// use shdp::prelude::common::{
///     registry::EVENT_REGISTRY_MSB,
///     event::{EventDecoder, EventEncoder},
///     bits::{BitDecoder, Frame}, error::Error
/// };
///
/// // Create a basic event for demonstration purposes.
///
/// pub struct MyEventRequest {
///     encoder: BitDecoder<Msb0>,
///     data: u32,
/// }
///
/// impl MyEventRequest {
///     pub fn new(decoder: BitDecoder<Msb0>) -> Self {
///         MyEventRequest {
///             encoder: decoder,
///             data: 0,
///         }
///     }
/// }
///
/// impl EventDecoder<Msb0> for MyEventRequest {
///     fn decode(&mut self, _: Frame<Msb0>) -> Result<(), Error> {
///         self.data = self.encoder.read_data(32)?;
///         Ok(())
///     }
///
///     fn get_responses(&self) -> Result<Vec<Box<dyn EventEncoder<Lsb0>>>, Error> {
///         Ok(vec![])
///     }
/// }
///
/// // Add the event to the registry.
///
/// EVENT_REGISTRY_MSB.lock().unwrap().add_event((1, 0x0000), |decoder| {
///    Box::new(MyEventRequest::new(decoder))
/// });
/// ```
pub struct EventRegistry<R: BitReversible> {
    events: HashMap<EventId, EventFn<R>>,
    listeners: HashMap<EventId, ListenerFn<R>>,
}

impl<R: BitReversible> EventRegistry<R> {
    fn new() -> Self {
        EventRegistry {
            events: HashMap::new(),
            listeners: HashMap::new(),
        }
    }

    ///
    /// Get the event from the registry.
    /// It returns the event if it exists, otherwise it returns None.
    ///
    /// # Arguments
    /// * `event_id` - The event id that represents the version and the event code.
    /// * `event_id.0` - The version of the event.
    /// * `event_id.1` - The event code.
    ///
    pub fn get_event(&self, event_id: EventId) -> Option<&EventFn<R>> {
        self.events.get(&event_id)
    }

    ///
    /// Get the listener from the registry.
    /// It returns a function that will be called when the event is received, otherwise it returns None.
    ///
    /// # Arguments
    /// * `event_id` - The event id that represents the version and the event code.
    /// * `event_id.0` - The version of the event.
    /// * `event_id.1` - The event code.
    ///
    pub fn get_listener(&self, event_id: EventId) -> Option<&ListenerFn<R>> {
        self.listeners.get(&event_id)
    }

    ///
    /// Add an event to the registry.
    ///
    /// # Arguments
    /// * `event_id` - The event id that represents the version and the event code.
    /// * `event_id.0` - The version of the event.
    /// * `event_id.1` - The event code.
    /// * `event_fn` - The event function that will be called when the event is received.
    ///
    pub fn add_event(&mut self, event_id: EventId, event_fn: EventFn<R>) {
        self.events.insert(event_id, event_fn);
    }

    ///
    /// Add a listener to the registry.
    ///
    /// # Arguments
    /// * `event_id` - The event id that represents the version and the event code.
    /// * `event_id.0` - The version of the event.
    /// * `event_id.1` - The event code.
    /// * `listener_fn` - The listener function that will be called when the event is received.
    ///
    pub fn add_listener(&mut self, event_id: EventId, listener_fn: ListenerFn<R>) {
        self.listeners.insert(event_id, listener_fn);
    }
}

lazy_static! {
    ///
    /// The event registry for the Msb0 (Most Significant Bit) order.
    /// It is used for server requests or client responses.
    ///
    pub static ref EVENT_REGISTRY_MSB: Arc<Mutex<EventRegistry<Msb0>>> =
        Arc::new(Mutex::new(EventRegistry::new()));

    ///
    /// The event registry for the Lsb0 (Least Significant Bit) order.
    /// It is used for server responses or client requests.
    ///
    pub static ref EVENT_REGISTRY_LSB: Arc<Mutex<EventRegistry<Lsb0>>> =
        Arc::new(Mutex::new(EventRegistry::new()));
}

static INIT: Once = Once::new();

#[ctor]
fn init() {
    INIT.call_once(|| {
        //
        // *** Server events ***
        //

        // ** Version 1 **

        #[cfg(any(feature = "tcp-server", feature = "ws-server"))]
        EVENT_REGISTRY_MSB
            .lock()
            .unwrap()
            .add_event((1, 0x0000), |decoder| {
                Box::new(ComponentNeedsRequest::new(decoder))
            });

        #[cfg(all(feature = "serde", any(feature = "tcp-server", feature = "ws-server")))]
        EVENT_REGISTRY_MSB
            .lock()
            .unwrap()
            .add_event((1, 0x0005), |decoder| {
                Box::new(InteractionRequest::new(decoder))
            });

        //
        // *** Client events ***
        //

        // ** Version 1 **

        #[cfg(any(feature = "tcp-client", feature = "ws-client"))]
        EVENT_REGISTRY_MSB
            .lock()
            .unwrap()
            .add_event((1, 0x0001), |decoder| {
                Box::new(HtmlFileResponse::new(decoder))
            });

        #[cfg(any(feature = "tcp-client", feature = "ws-client"))]
        EVENT_REGISTRY_LSB
            .lock()
            .unwrap()
            .add_event((1, 0x0002), |decoder| Box::new(ErrorResponse::new(decoder)));

        #[cfg(any(feature = "tcp-client", feature = "ws-client"))]
        EVENT_REGISTRY_LSB
            .lock()
            .unwrap()
            .add_event((1, 0x0003), |decoder| {
                Box::new(ComponentNeedsResponse::new(decoder))
            });

        #[cfg(any(feature = "tcp-client", feature = "ws-client"))]
        EVENT_REGISTRY_MSB
            .lock()
            .unwrap()
            .add_event((1, 0x0004), |decoder| {
                Box::new(FullFyveResponse::new(decoder))
            });

        #[cfg(all(any(feature = "tcp-client", feature = "ws-client"), feature = "serde"))]
        EVENT_REGISTRY_LSB
            .lock()
            .unwrap()
            .add_event((1, 0x0006), |decoder| {
                Box::new(InteractionResponse::new(decoder))
            });
    })
}
