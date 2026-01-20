#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    /// TODO: Remove
    pub struct PacketPlaceholder;
}
