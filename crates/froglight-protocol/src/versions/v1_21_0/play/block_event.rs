use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct BlockEventPacket {
    pub position: BlockPosition,
    pub event_type: u8,
    pub event_data: u8,
    #[frog(var)]
    pub block_type: u32,
}
