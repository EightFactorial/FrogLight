#![cfg_attr(feature = "nightly", feature(portable_simd))]
#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "bevy")]
pub use bevy_math;
pub use glam;

pub mod functions;
#[cfg(any(all(feature = "libm", feature = "once_cell"), feature = "std"))]
pub mod table;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::functions::*;
    #[cfg(any(all(feature = "libm", feature = "once_cell"), feature = "std"))]
    pub use crate::table::{cos, sin, sin_cos};
}
