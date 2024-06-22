use froglight_macros::FrogReadWrite;

use crate::common::{ChunkPosition, UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct LightUpdatePacket {
    pub position: ChunkPosition,
    // TODO: Implement LightData
    pub light_data: UnsizedBuffer,
}
