#![doc = include_str!("../README.md")]
#![allow(clippy::std_instead_of_alloc, reason = "Requires the standard library")]
#![allow(clippy::std_instead_of_core, reason = "Requires the standard library")]

#[cfg(feature = "bevy")]
pub mod bevy;
pub mod connection;
pub mod event;
pub use facet_minecraft;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "bevy")]
    pub use crate::bevy::{ClientConnection, ClientboundMessage, ServerboundMessage};
    pub use crate::event::enums::{ClientboundEventEnum, ServerboundEventEnum};
}
