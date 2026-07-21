#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "facet")]
pub mod facet;
pub mod types;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "facet")]
    pub use crate::facet::{
        deserialize::{DeserializeNbt, functions::*},
        serialize::{SerializeNbt, functions::*},
    };
    pub use crate::types::{
        indexed::{
            IndexedNbt,
            alloc::{IndexedNbtCow, IndexedNbtSlice},
            entry::{IndexedEntry, IndexedValue},
        },
        structured::Nbt,
    };
}
