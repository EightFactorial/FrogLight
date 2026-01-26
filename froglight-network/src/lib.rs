#![doc = include_str!("../README.md")]

#[cfg(feature = "bevy")]
pub mod bevy;
pub mod connection;
pub mod event;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "bevy")]
    pub use crate::bevy::{ClientConnection, ClientboundMessage, ServerboundMessage};
    pub use crate::event::{ClientboundEventEnum, ServerboundEventEnum};
}
