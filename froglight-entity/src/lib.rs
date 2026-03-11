#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod atomic;
#[cfg(feature = "bevy")]
pub mod bevy;
pub mod entity;
pub mod generated;
pub mod storage;
pub mod types;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        entity::{EntityBundle, EntityType},
        generated::{
            component as entity_data,
            entity::{self, VanillaEntity},
        },
        version::EntityVersion,
    };
}
