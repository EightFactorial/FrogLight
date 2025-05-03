#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "io")]
pub use froglight_io as io;
pub use froglight_packet as packet;

pub mod connection;
#[cfg(feature = "io")]
pub mod froglight_io;

pub mod prelude {
    //! Re-exports of commonly used types, traits, and macros.

    #[cfg(feature = "io")]
    pub use froglight_io::prelude::*;
    pub use froglight_packet::prelude::*;
    #[cfg(feature = "resolver")]
    pub use froglight_resolver::prelude::*;

    pub use crate::connection::{ClientConnection, Connection, ServerConnection};
}
