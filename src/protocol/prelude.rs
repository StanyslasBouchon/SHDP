pub mod common {
    pub use crate::protocol::args::Arg;
    pub use crate::protocol::versions::Version;

    pub mod bits {
        pub use crate::protocol::managers::bits::decoder::*;
        pub use crate::protocol::managers::bits::encoder::*;

        pub mod util {
            pub use crate::protocol::managers::bits::prelude::*;
        }
    }

    pub mod event {
        pub use crate::protocol::managers::event::*;
    }

    pub mod error {
        pub use crate::protocol::errors::*;
    }

    pub mod registry {
        pub use crate::protocol::managers::registry::*;
    }
}

///
/// A RemoteId is a tuple of two strings, the first being the remote's host address and the second being the remote's port.
///
pub type RemoteId = (String, String);
