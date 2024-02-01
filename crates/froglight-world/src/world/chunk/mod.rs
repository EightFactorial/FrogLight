//! A [`Chunk`] is a 16x16xN section of blocks.

use std::sync::Arc;

use bevy_ecs::component::Component;
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
    /// The height of the chunk.
    pub height: usize,

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

    /// Creates a new empty [`Chunk`] with the given height.
    #[must_use]
    pub fn new_empty(height: usize) -> Self {
        Self {
            height,
            sections: Arc::new(RwLock::new(Vec::new())),
            heightmaps: Arc::new(RwLock::new(HeightMaps::default())),
        }
    }

    /// Creates a new [`Chunk`] with the given sections.
    ///
    /// This calculates the heightmaps from the sections.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::world::{Chunk, Section};
    ///
    /// // Create a new chunk with 24 sections.
    /// let chunk = Chunk::new(vec![Section::default(); 24]);
    ///
    /// // The chunk has a height of (24 x 16) 384.
    /// assert_eq!(chunk.height, 384);
    /// ```
    #[must_use]
    pub fn new(sections: Vec<Section>) -> Self {
        // TODO: Calculate heightmaps from sections.
        let heightmaps = HeightMaps::default();

        Self {
            height: sections.len() * Section::HEIGHT,
            sections: Arc::new(RwLock::new(sections)),
            heightmaps: Arc::new(RwLock::new(heightmaps)),
        }
    }

    /// Creates a new [`Chunk`] with the given sections and heightmaps.
    #[must_use]
    pub fn new_with_heightmaps(sections: Vec<Section>, heightmaps: HeightMaps) -> Self {
        Self {
            height: sections.len() * Section::HEIGHT,
            sections: Arc::new(RwLock::new(sections)),
            heightmaps: Arc::new(RwLock::new(heightmaps)),
        }
    }

    /// Decodes a [`Chunk`] from a buffer.
    ///
    /// # Errors
    /// If the data in the buffer is invalid, an error will be returned.
    pub fn read_from_buffer(
        height: usize,
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
