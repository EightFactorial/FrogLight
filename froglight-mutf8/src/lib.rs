#![doc = include_str!("../README.md")]
#![allow(unused_features, reason = "WIP")]
#![cfg_attr(feature = "nightly", feature(portable_simd))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "simd")]
pub mod simd;

pub mod types;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "alloc")]
    pub use crate::types::MString;
    pub use crate::{mutf8, types::MStr};
}
