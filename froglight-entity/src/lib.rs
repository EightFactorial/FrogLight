#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod atomic;
#[cfg(feature = "bevy")]
pub mod bevy;
pub mod entity;
pub mod generated;
pub mod storage;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        entity::{EntityBundle, EntityType},
        generated::entity::{self, VanillaEntity},
        version::EntityVersion,
    };
}
