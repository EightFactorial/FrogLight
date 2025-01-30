#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod chunk;
pub mod section;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        chunk::{ArrayChunk, VecChunk},
        section::Section,
    };
}
