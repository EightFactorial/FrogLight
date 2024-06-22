use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

use crate::common::BlockPosition;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct BlockEntityUpdatePacket {
    pub position: BlockPosition,
    #[frog(var)]
    pub entity_type: u32,
    pub data: Nbt,
}
