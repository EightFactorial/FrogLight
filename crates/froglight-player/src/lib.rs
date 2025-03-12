#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod chat;
pub mod profile;
pub mod username;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        chat::{signature::MessageSignatureCtx, text::Text},
        profile::PlayerProfile,
        username::PlayerUsername,
    };
}
