#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod profile;
pub mod username;

#[cfg(feature = "io")]
mod io;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{profile::PlayerProfile, username::PlayerUsername};
}
