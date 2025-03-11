#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod agent;
pub mod connection;
pub mod types;
pub mod version;

#[cfg(feature = "resolver")]
pub mod resolver;

pub mod prelude {
    //! Re-exports of commonly used types, traits, and macros.

    pub use froglight_io::prelude::*;

    #[cfg(feature = "resolver")]
    pub use crate::resolver::FroglightResolver;
    pub use crate::{
        agent::FroglightAgent,
        connection::{ClientConnection, ServerConnection},
        version::state::{Config, Handshake, Login, Play, State, Status, ValidState},
    };
}
