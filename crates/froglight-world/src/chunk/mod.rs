//! [`Chunk`], [`ChunkStorage`], [`Section`], and [`SectionData`].

mod palette;
// pub use palette::SectionPalette;

mod section;
pub use section::{Section, SectionData};

mod storage;
pub use storage::{ArrayChunkStorage, ChunkStorage, VecChunkStorage};
