#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;

pub mod message;
pub mod text;
pub mod translate;

#[cfg(feature = "crypto")]
pub mod chat;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "crypto")]
    pub use crate::chat::MessageSignatureCtx;
    pub use crate::text::{
        FormattedText,
        content::TextContent,
        interaction::TextInteraction,
        style::{IntegerColor, PresetColor, TextColor, TextStyle},
    };
}
