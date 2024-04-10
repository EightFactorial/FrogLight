#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod chunk;
pub use chunk::Chunk;

mod container;
pub use container::{BiomeContainer, BiomeStorage, BlockContainer, BlockStorage, Container};

mod iterators;
pub use iterators::{ChunkBlockIter, SectionBlockIter};

mod palette;
pub use palette::ContainerPalette;

mod section;
pub use section::ChunkSection;

#[cfg(test)]
mod tests;
