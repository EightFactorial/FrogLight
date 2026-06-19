#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "facet")]
pub mod facet;
pub mod types;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::types::{indexed::IndexedNbt, structured::Nbt};
}
