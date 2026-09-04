#![cfg_attr(feature = "nightly", feature(portable_simd))]
#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod functions;
#[cfg(any(
    all(feature = "libm", feature = "once_cell", feature = "critical-section"),
    feature = "std"
))]
pub mod table;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::functions::*;
    #[cfg(any(
        all(feature = "libm", feature = "once_cell", feature = "critical-section"),
        feature = "std"
    ))]
    pub use crate::table::*;
}
