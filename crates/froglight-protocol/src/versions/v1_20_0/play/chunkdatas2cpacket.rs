use froglight_macros::FrogReadWrite;

use crate::common::{ChunkDataPacket, ChunkPosition, UnsizedByteBuffer};

#[derive(Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ChunkDataS2CPacket {
    pub position: ChunkPosition,
    pub chunk_data: ChunkDataPacket,
    pub light_data: UnsizedByteBuffer,
}

#[allow(clippy::missing_fields_in_debug)]
impl std::fmt::Debug for ChunkDataS2CPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkDataS2CPacket")
            .field("position", &self.position)
            .field("chunk_data", &self.chunk_data)
            .finish()
    }
}
