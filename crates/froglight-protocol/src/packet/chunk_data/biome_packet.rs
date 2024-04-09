use froglight_macros::FrogReadWrite;

use crate::common::ChunkPosition;

/// A packet containing biome data for a chunk.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct BiomeDataPacket {
    /// The position of the chunk.
    pub position: ChunkPosition,
    /// The biome data.
    pub data: Vec<u8>,
}
