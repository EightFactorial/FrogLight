//! A [`Chunk`] is a 16x16xN section of blocks.

use std::sync::Arc;

use bevy_ecs::component::Component;
use parking_lot::RwLock;
use thiserror::Error;

mod heightmap;
use heightmap::HeightMaps;

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
#[derive(Debug, Clone, Component)]
pub struct Chunk<const HEIGHT: usize>
where
    [(); HEIGHT / Section::HEIGHT]:,
{
    /// The sections in the chunk.
    pub sections: Arc<RwLock<[Section; HEIGHT / Section::HEIGHT]>>,
    /// The heightmaps of the chunk.
    pub heightmaps: Arc<RwLock<HeightMaps>>,
}

impl<const HEIGHT: usize> Default for Chunk<HEIGHT>
where
    [(); HEIGHT / Section::HEIGHT]:,
{
    fn default() -> Self {
        let sections = core::array::from_fn(|_| Section::default());
        Self {
            sections: Arc::new(RwLock::new(sections)),
            heightmaps: Arc::new(RwLock::new(HeightMaps::default())),
        }
    }
}

impl<const HEIGHT: usize> Chunk<HEIGHT>
where
    [(); HEIGHT / Section::HEIGHT]:,
{
    /// The width of a [`Chunk`].
    pub const WIDTH: usize = 16;
    /// The depth of a [`Chunk`].
    pub const DEPTH: usize = 16;
    /// The height of a [`Chunk`].
    pub const HEIGHT: usize = HEIGHT;

    /// The total volume of a [`Chunk`].
    pub const VOLUME: usize = Self::WIDTH * Self::DEPTH * Self::HEIGHT;

    /// The total number of [`Section`]s in a [`Chunk`].
    pub const SECTION_COUNT: usize = HEIGHT / Section::HEIGHT;

    /// Decodes a [`Chunk`] from a buffer.
    ///
    /// # Errors
    /// If the data in the buffer is invalid, an error will be returned.
    pub fn read_from_buffer(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ChunkDecodeError> {
        let sections = core::array::try_from_fn(|_| Section::decode(buf))?;
        let heightmaps = HeightMaps::decode(buf)?;

        Ok(Self {
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

#[test]
fn test_chunk_section_count() {
    // The Overworld has a height of 384, so it has 24 sections.
    assert_eq!(Chunk::<384>::SECTION_COUNT, 24);

    // The Nether and End have a height of 256, so they have 16 sections.
    assert_eq!(Chunk::<256>::SECTION_COUNT, 16);
}
