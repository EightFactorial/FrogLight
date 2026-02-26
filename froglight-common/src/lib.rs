#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "bevy")]
pub mod bevy;
pub mod entity;
pub mod identifier;
pub mod impossible;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "bevy")]
    pub use crate::bevy::{relationship::EntityOfInstance, world::WorldInstance};
    pub use crate::{
        entity::{EntityId, EntityUuid},
        identifier::Identifier,
        version::*,
    };
}
