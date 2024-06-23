use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct QueryBlockNbtPacket {
    #[frog(var)]
    pub id: u32,
    pub position: BlockPosition,
}
