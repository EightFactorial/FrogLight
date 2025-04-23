#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod chat;
pub mod player;
pub mod text;
pub mod translate;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        chat::signature::MessageSignatureCtx,
        player::{profile::PlayerProfile, username::PlayerUsername, uuid::PlayerUuid},
        text::{FormattedText, TextColor, TextFormatting},
        translate::TextTranslations,
    };
}
