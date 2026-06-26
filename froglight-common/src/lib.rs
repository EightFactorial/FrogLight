#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

pub mod identifier;
pub mod impossible;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{identifier::Identifier, version::*};
}
