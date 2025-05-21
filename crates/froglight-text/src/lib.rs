#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
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
        content::TextContent,
        interaction::TextInteraction,
        style::{IntegerColor, PresetColor, TextColor, TextStyle},
        FormattedText,
    };
}
