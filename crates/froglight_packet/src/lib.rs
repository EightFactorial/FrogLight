#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;
extern crate facet_core as facet;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "network")]
pub mod network;
pub mod state;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "network")]
    pub use crate::network::{ClientConnection, Connection, ServerConnection};
    pub use crate::state::*;
}
