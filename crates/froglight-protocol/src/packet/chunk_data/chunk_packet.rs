use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

use super::BlockEntity;

/// A chunk of data representing a section of the world.
#[derive(Clone, PartialEq, FrogReadWrite)]
pub struct ChunkDataBuffer {
    /// The heightmap data for the chunk.
    pub heightmaps: Nbt,
    /// The chunk blocks and biomes.
    pub data: Vec<u8>,
    /// The block entities in the chunk.
    pub entities: Vec<BlockEntity>,
}

#[allow(clippy::missing_fields_in_debug)]
impl std::fmt::Debug for ChunkDataBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkDataBuffer").field("entities", &self.entities).finish()
    }
}
