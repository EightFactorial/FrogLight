#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod chunk;
pub mod section;
#[cfg(all(feature = "bevy", feature = "block"))]
pub mod storage;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(all(feature = "bevy", feature = "block"))]
    pub use crate::storage::{ChunkHandle, ChunkStorage, End, Nether, Overworld};
    pub use crate::{
        chunk::{ArrayChunk, VecChunk},
        section::Section,
    };
}
