#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "aabb")]
pub mod aabb;
#[cfg(feature = "uuid")]
pub mod entity;
pub mod identifier;
pub mod impossible;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "uuid")]
    pub use crate::entity::{EntityId, EntityUuid};
    pub use crate::{identifier::Identifier, version::*};
}
