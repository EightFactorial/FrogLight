#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod chunk;
pub mod position;

pub mod prelude {
    //! Re-exports of common types and traits.

    pub use crate::{
        chunk::Section,
        position::{BlockPos, ChunkPos, DimensionPos},
    };
}
