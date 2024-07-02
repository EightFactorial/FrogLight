use froglight_macros::FrogReadWrite;

use crate::common::ChunkPosition;

#[derive(Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChunkDataPacket {
    pub position: ChunkPosition,
    // TODO: Fix Nbt Error?
    // pub chunk_data: crate::packet::ChunkDataBuffer,
    pub chunk_data: crate::common::UnsizedBuffer,
    // pub light_data: UnsizedBuffer,
}

#[allow(clippy::missing_fields_in_debug)]
impl std::fmt::Debug for ChunkDataPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkDataBuffer")
            .field("position", &self.position)
            // .field("chunk_data", &self.chunk_data)
            .finish()
    }
}
