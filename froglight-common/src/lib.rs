#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "aabb")]
pub mod aabb;
#[cfg(feature = "bevy")]
pub mod bevy;
#[cfg(feature = "uuid")]
pub mod entity;
pub mod identifier;
pub mod impossible;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "bevy")]
    pub use crate::bevy::{
        instance::WorldInstance,
        query::{InInstance, Instance, OnInstance},
        relationship::EntityOfInstance,
    };
    #[cfg(feature = "uuid")]
    pub use crate::entity::{EntityId, EntityUuid};
    pub use crate::{identifier::Identifier, version::*};
}
