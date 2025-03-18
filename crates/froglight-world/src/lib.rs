#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "bevy")]
pub mod bevy;
pub mod chunk;
pub mod position;

pub mod prelude {
    //! Re-exports of common types and traits.

    pub use crate::{
        chunk::{Chunk, Section},
        position::{BlockPos, ChunkPos, DimensionPos},
    };
}
