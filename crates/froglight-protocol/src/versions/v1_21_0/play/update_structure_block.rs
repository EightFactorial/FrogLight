use froglight_macros::FrogReadWrite;

use crate::common::{BlockPosition, UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct UpdateStructureBlockPacket {
    pub position: BlockPosition,
    // TODO: Implement StructureBlockData
    pub data: UnsizedBuffer,
}
