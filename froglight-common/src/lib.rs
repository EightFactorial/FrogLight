#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod entity;
pub mod identifier;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        entity::{EntityId, EntityUuid},
        identifier::Identifier,
        version::*,
    };
}
