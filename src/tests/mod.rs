mod client;
mod server;

mod frame_decoding;
mod frame_error_decoding;

#[cfg(all(
    any(feature = "tcp-client", feature = "ws-client"),
    any(feature = "tcp-server", feature = "ws-server")
))]
mod t0x0000_event_decoding_full_track;

#[cfg(all(
    any(feature = "tcp-client", feature = "ws-client"),
    any(feature = "tcp-server", feature = "ws-server")
))]
mod t0x0001_event_decoding_full_track;

#[cfg(all(
    any(feature = "tcp-client", feature = "ws-client"),
    any(feature = "tcp-server", feature = "ws-server")
))]
mod t0x0002_event_decoding_full_track;

#[cfg(all(
    any(feature = "tcp-client", feature = "ws-client"),
    any(feature = "tcp-server", feature = "ws-server")
))]
mod t0x0003_event_decoding_full_track;

#[cfg(all(
    any(feature = "tcp-client", feature = "ws-client"),
    any(feature = "tcp-server", feature = "ws-server")
))]
mod t0x0004_event_decoding_full_track;

#[cfg(all(
    any(feature = "tcp-client", feature = "ws-client"),
    any(feature = "tcp-server", feature = "ws-server")
))]
mod t0x0005_event_decoding_full_track;

#[cfg(all(
    any(feature = "tcp-client", feature = "ws-client"),
    any(feature = "tcp-server", feature = "ws-server")
))]
mod t0x0006_event_decoding_full_track;
