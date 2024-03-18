use froglight_macros::FrogReadWrite;

use crate::common::{ChunkPosition, UnsizedByteBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct LightUpdateS2CPacket {
    pub pos: ChunkPosition,
    pub data: UnsizedByteBuffer,
}
