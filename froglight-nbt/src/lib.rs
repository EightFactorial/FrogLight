#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod types;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::types::borrowed::{
        IndexedNbtMut, IndexedNbtRef,
        compound::{IndexedCompoundMut, IndexedCompoundRef, IndexedEntry},
    };
}
