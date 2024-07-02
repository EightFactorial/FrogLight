use froglight_macros::FrogReadWrite;
use glam::IVec2;

use crate::{common::UnsizedBuffer, packet::ChunkDataBuffer};

#[derive(Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChunkDataPacket {
    pub position: IVec2,
    pub chunk_data: ChunkDataBuffer,
    pub light_data: UnsizedBuffer,
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
