//! A [`Section`] is a 16x16x16 section of a [`Chunk`](super::Chunk).

use std::io::Cursor;

use froglight_protocol::io::FrogRead;

mod container;
pub use container::{BiomeContainer, BlockContainer, Container, ContainerType};

mod palette;
pub use palette::Palette;

use super::{chunk::ChunkDecodeError, Chunk};

/// A [`Section`] is a 16x16x16 section of a [`Chunk`](super::Chunk).
///
/// A [`Section`] contains a counter for the number of non-air blocks in the
/// section, and [`Containers`](super::container::Container) for blocks and
/// biomes.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Section {
    /// The number of non-air blocks in the section.
    pub block_count: u16,
    /// The block data stored in the section.
    pub blocks: Container<BlockContainer>,
    /// The biome data stored in the section.
    pub biomes: Container<BiomeContainer>,
}

impl Section {
    /// The width of a [`Section`].
    pub const WIDTH: usize = Chunk::<0>::WIDTH;
    /// The depth of a [`Section`].
    pub const DEPTH: usize = Chunk::<0>::DEPTH;
    /// The height of a [`Section`].
    pub const HEIGHT: usize = 16;

    /// The total volume of a [`Section`].
    pub const VOLUME: usize = Self::WIDTH * Self::DEPTH * Self::HEIGHT;

    /// Decodes a [`Section`] from a buffer.
    pub(crate) fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, ChunkDecodeError> {
        Ok(Self {
            block_count: u16::frog_read(buf)?,
            blocks: Container::decode(buf)?,
            biomes: Container::decode(buf)?,
        })
    }
}
