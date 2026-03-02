#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "bevy")]
pub mod bevy;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    /// TODO: Delete me
    pub struct PlaceholderEntity;
}
