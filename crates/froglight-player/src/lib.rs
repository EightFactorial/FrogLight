#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod chat;
pub mod player;
pub mod text;
pub mod translate;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "std")]
    pub use crate::chat::signature::MessageSignatureCtx;
    pub use crate::{
        player::{profile::PlayerProfile, username::PlayerUsername, uuid::PlayerUuid},
        text::{FormattedText, TextColor, TextFormatting},
        translate::TextTranslations,
    };
}
