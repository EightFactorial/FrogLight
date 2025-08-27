#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod actor;
pub mod backend;
pub mod builder;
pub mod client;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        actor::{ApiPublicKeys, PlayerProfile, ServerId},
        client::{ApiAuthClient, ApiClient},
    };
}
