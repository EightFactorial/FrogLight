#![cfg_attr(feature = "nightly", feature(portable_simd))]
#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    /// TODO: Delete Me
    pub struct PhysicsPlaceholder;
}
