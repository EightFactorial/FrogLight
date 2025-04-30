#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod entity;
pub mod identifier;
pub mod vanilla;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use froglight_macros::Version;

    pub use crate::{
        entity::{EntityId, EntityUuid},
        identifier::Identifier,
        vanilla::Vanilla,
        version::Version,
    };
}
