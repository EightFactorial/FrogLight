//! A [`Chunk`] is a 16x16xN section of blocks.

use std::sync::Arc;

use bevy_ecs::component::Component;
use bevy_log::warn;
use froglight_core::data::ChunkBlockPosition;
use parking_lot::RwLock;
use thiserror::Error;

mod heightmap;
pub use heightmap::HeightMaps;

use super::section::Section;

/// A [`Chunk`] is a 16x16xN section of blocks.
///
/// Each `World` is made up of `Chunks`,
/// with each `World` having it's own height.
///
/// Heights:
/// - Overworld: 384 (-64 to 320)
/// - Nether: 256 (0 to 256)
/// - End: 256 (0 to 256)
#[derive(Debug, Default, Clone, Component)]
pub struct Chunk {
    /// The absolute height of the chunk.
    ///
    /// This is equivalent to the highest y-coordinate minus the offset.
    pub height: usize,

    /// The height offset of the chunk.
    ///
    /// This is equivalent to the lowest y-coordinate.
    pub offset: isize,

    /// The sections in the chunk.
    pub sections: Arc<RwLock<Vec<Section>>>,
    /// The heightmaps of the chunk.
    pub heightmaps: Arc<RwLock<HeightMaps>>,
}

impl Chunk {
    /// The width of a [`Chunk`].
    pub const WIDTH: usize = 16;
    /// The depth of a [`Chunk`].
    pub const DEPTH: usize = 16;

    /// The total volume of the [`Chunk`].
    #[must_use]
    pub const fn volume(&self) -> usize { Self::WIDTH * Self::DEPTH * self.height }

    /// Creates a new empty [`Chunk`] with the given height.
    #[must_use]
    pub fn new_empty(height: usize, offset: isize) -> Self {
        Self {
            height,
            offset,
            sections: Arc::new(RwLock::new(Vec::with_capacity(height / Section::HEIGHT))),
            heightmaps: Arc::new(RwLock::new(HeightMaps::default())),
        }
    }

    /// Creates a new [`Chunk`] with the given sections.
    ///
    /// This calculates the world height from the number of sections.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::world::{Chunk, Section};
    ///
    /// // Create a new chunk with 24 sections.
    /// let chunk = Chunk::new(vec![Section::default(); 24], -64);
    ///
    /// // The chunk has a height of (24 x 16) 384.
    /// assert_eq!(chunk.height, 384);
    /// ```
    #[must_use]
    pub fn new(sections: Vec<Section>, offset: isize) -> Self {
        // TODO: Calculate heightmaps from sections.
        let heightmaps = HeightMaps::default();

        Self {
            height: sections.len() * Section::HEIGHT,
            offset,
            sections: Arc::new(RwLock::new(sections)),
            heightmaps: Arc::new(RwLock::new(heightmaps)),
        }
    }

    /// Creates a new [`Chunk`] with the given sections and heightmaps.
    ///
    /// This calculates the world height from the number of sections.
    #[must_use]
    pub fn new_with_heightmaps(
        sections: Vec<Section>,
        offset: isize,
        heightmaps: HeightMaps,
    ) -> Self {
        Self {
            height: sections.len() * Section::HEIGHT,
            offset,
            sections: Arc::new(RwLock::new(sections)),
            heightmaps: Arc::new(RwLock::new(heightmaps)),
        }
    }

    /// Gets the block at the given position in the chunk.
    ///
    /// Returns [`None`] if the position is out of bounds.
    pub fn get_block(&self, pos: &ChunkBlockPosition) -> Option<usize> {
        let section_index = pos.y() / Section::HEIGHT;
        if let Some(section) = self.sections.read().get(section_index) {
            let pos = pos.into();
            return Some(section.get_block(&pos));
        }

        warn!("Attempted to get block from non-existent section");
        None
    }

    /// Sets the block at the given position in the chunk.
    ///
    /// Returns the previous block id at the position.
    ///
    /// Returns [`None`] if the position is out of bounds.
    pub fn set_block(&mut self, pos: &ChunkBlockPosition, value: usize) -> Option<usize> {
        let section_index = pos.y() / Section::HEIGHT;
        if let Some(section) = self.sections.write().get_mut(section_index) {
            let pos = pos.into();
            return Some(section.set_block(&pos, value));
        }

        warn!("Attempted to set block in non-existent section");
        None
    }

    /// Decodes a [`Chunk`] from a buffer.
    ///
    /// # Errors
    /// If the data in the buffer is invalid, an error will be returned.
    pub fn read_from_buffer(
        height: usize,
        offset: isize,
        buf: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, ChunkDecodeError> {
        // Decode the sections.
        let section_count = height / Section::HEIGHT;

        let mut sections = Vec::with_capacity(section_count);
        for _ in 0..section_count {
            sections.push(Section::decode(buf)?);
        }

        // Decode the heightmaps.
        let heightmaps = HeightMaps::decode(height, buf)?;

        Ok(Self {
            height,
            offset,
            sections: Arc::new(RwLock::new(sections)),
            heightmaps: Arc::new(RwLock::new(heightmaps)),
        })
    }
}

/// A [`ChunkDecodeError`] is an error that occurs when decoding a [`Chunk`].
#[derive(Debug, Error)]
pub enum ChunkDecodeError {
    /// An error occurred while reading the chunk data.
    #[error(transparent)]
    ReadError(#[from] froglight_protocol::io::ReadError),
    /// An error occurred while parsing nbt data.
    #[error(transparent)]
    NbtError(#[from] simdnbt::DeserializeError),
    /// An error occurred while reading section data.
    #[error("Could not decode bitvec")]
    BitVec,
}
