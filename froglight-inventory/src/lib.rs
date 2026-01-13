#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "bevy")]
pub mod bevy;
pub mod inventory;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    /// TODO: Remove
    pub struct InventoryPlaceholder;
}
