#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub use froglight_io as io;

pub mod connection;
pub mod types;
pub mod version;

pub mod prelude {
    //! Re-exports of commonly used types, traits, and macros.

    pub use froglight_io::prelude::*;
    #[cfg(feature = "resolver")]
    pub use froglight_resolver::prelude::*;

    pub use crate::{
        connection::{ClientConnection, ServerConnection},
        version::state::{Config, Handshake, Login, Play, State, Status, ValidState},
    };
}
