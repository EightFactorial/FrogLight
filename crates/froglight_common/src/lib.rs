#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;

pub mod atomic;
pub mod digest;
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
