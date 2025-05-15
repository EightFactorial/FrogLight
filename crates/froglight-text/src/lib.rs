#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod chat;
pub mod text;
pub mod translate;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        chat::MessageSignatureCtx,
        text::{
            FormattedText,
            style::{IntegerColor, PresetColor, TextColor, TextStyle},
        },
    };
}
