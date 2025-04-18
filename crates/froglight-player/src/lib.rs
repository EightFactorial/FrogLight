#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod chat;
pub mod component;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        chat::{
            signature::MessageSignatureCtx,
            text::{FormattedText, TextColor, TextFormatting},
        },
        component::{profile::PlayerProfile, username::PlayerUsername, uuid::PlayerUuid},
    };
}
