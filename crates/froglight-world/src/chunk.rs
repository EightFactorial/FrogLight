use std::sync::Arc;

use froglight_protocol::{
    common::ChunkBlockPosition,
    protocol::{FrogRead, ReadError},
};
use parking_lot::RwLock;

use crate::{ChunkBlockIter, ChunkSection};

/// A [`Chunk`] is a `16 x Y x 16 (X,Y,Z)`  section of blocks.
///
/// Because heights and offsets vary between `Worlds`, [`Chunks`](Self)
/// belonging to different `Worlds` can have a different amount of
/// [`ChunkSections`](ChunkSection).
///
/// Example:
/// - `minecraft:overworld`: `384 (-64 to 320)`
/// - `minecraft:the_nether`: `256 (0 to 256)`
/// - `minecraft:the_end`: `256 (0 to 256)`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct Chunk {
    /// The maximum height of the chunk.
    pub max_height: u32,
    /// The height offset of the chunk.
    pub height_offset: i32,

    /// The chunk's sections.
    pub sections: Arc<RwLock<Vec<ChunkSection>>>,
}

impl Chunk {
    /// The width of a [`Chunk`].
    pub const WIDTH: u32 = 16u32;
    /// The depth of a [`Chunk`].
    pub const DEPTH: u32 = 16u32;

    /// Returns the volume of the [`Chunk`].
    #[must_use]
    pub const fn volume(&self) -> u32 { Self::WIDTH * self.max_height * Self::DEPTH }

    /// Returns the height of the [`Chunk`],
    /// based on the maximum height and height offset.
    #[must_use]
    pub const fn height(&self) -> u32 { Self::internal_height(self.max_height, self.height_offset) }

    #[allow(clippy::cast_sign_loss)]
    const fn internal_height(max_height: u32, height_offset: i32) -> u32 {
        max_height.wrapping_sub(height_offset as u32)
    }

    /// Creates a new empty [`Chunk`] with the given height.
    #[must_use]
    pub fn new_empty(max_height: u32, height_offset: i32) -> Self {
        let section_count = Self::internal_height(max_height, height_offset) / ChunkSection::HEIGHT;
        Self {
            max_height,
            height_offset,
            sections: Arc::new(RwLock::new(vec![ChunkSection::default(); section_count as usize])),
        }
    }

    /// Returns an iterator over the blocks of the [`Chunk`].
    #[must_use]
    pub fn block_iter(&self) -> ChunkBlockIter<'_> { ChunkBlockIter::new(self) }

    /// Returns the `Block ID` at the given position.
    #[must_use]
    pub fn get_block(&self, position: ChunkBlockPosition) -> Option<u32> {
        let section_index = position.y() / ChunkSection::HEIGHT;
        self.sections.read().get(section_index as usize).map(|s| s.get_block(position.into()))
    }

    /// Sets the `Biome ID` at the given position.
    #[must_use]
    pub fn get_biome(&self, position: ChunkBlockPosition) -> Option<u32> {
        let section_index = position.y() / ChunkSection::HEIGHT;
        self.sections.read().get(section_index as usize).map(|s| s.get_biome(position.into()))
    }
}

impl Chunk {
    /// Reads a [`Chunk`] from the given buffer.
    ///
    /// Requires knowing the maximum height and height offset of the chunk.
    ///
    /// # Errors
    /// If the chunk could not be read from the buffer.
    pub fn read_from(
        max_height: u32,
        height_offset: i32,
        buf: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, ReadError> {
        let section_count = Self::internal_height(max_height, height_offset) / ChunkSection::HEIGHT;

        let mut sections = Vec::with_capacity(section_count as usize);
        for _ in 0..section_count {
            sections.push(ChunkSection::fg_read(buf)?);
        }

        // TODO: Read heightmaps

        Ok(Self { max_height, height_offset, sections: Arc::new(RwLock::new(sections)) })
    }
}
