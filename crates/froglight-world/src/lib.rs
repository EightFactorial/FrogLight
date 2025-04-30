#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "bevy")]
pub mod bevy;
pub mod chunk;
pub mod position;

pub mod prelude {
    //! Re-exports of common types and traits.

    #[cfg(feature = "bevy")]
    pub use crate::bevy::{ChunkPositionMap, EntityPositionMap};
    pub use crate::{
        chunk::{Chunk, Section},
        position::{BlockPos, ChunkPos, DimensionPos},
    };
}
