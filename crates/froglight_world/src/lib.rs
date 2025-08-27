#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;

pub mod chunk;
pub mod palette;
pub mod position;
#[cfg(feature = "bevy")]
pub mod reflect;
pub mod section;
pub mod storage;
#[cfg(feature = "block")]
pub mod versioned;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        chunk::Chunk,
        position::{BlockPosition, ChunkPosition},
        section::Section,
    };
}
