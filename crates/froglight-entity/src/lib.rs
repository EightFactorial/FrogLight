#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod player;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::player::{profile::PlayerProfile, username::PlayerUsername, uuid::PlayerUuid};
}
