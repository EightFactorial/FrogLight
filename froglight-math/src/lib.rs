#![cfg_attr(feature = "nightly", feature(core_float_math))]
#![cfg_attr(feature = "nightly", feature(portable_simd))]
#![cfg_attr(
    feature = "nightly",
    allow(unused_features, reason = "`core_float_math` used if `no_std`")
)]
#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod functions;

#[cfg(any(
    all(feature = "libm", feature = "once_cell", feature = "critical-section"),
    feature = "std",
))]
pub mod table;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::functions::*;
    #[cfg(any(
        all(feature = "libm", feature = "once_cell", feature = "critical-section"),
        feature = "std",
    ))]
    pub use crate::table::{cos, sin, sin_cos};
}
