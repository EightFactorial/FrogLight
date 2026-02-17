#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

#[cfg(feature = "bevy")]
pub mod bevy;

pub mod profile;
pub mod username;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{profile::PlayerProfile, username::Username};
}
