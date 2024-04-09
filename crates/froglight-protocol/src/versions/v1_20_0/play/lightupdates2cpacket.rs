use froglight_macros::FrogReadWrite;

use crate::common::{ChunkPosition, UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct LightUpdateS2CPacket {
    pub pos: ChunkPosition,
    pub data: UnsizedBuffer,
}
