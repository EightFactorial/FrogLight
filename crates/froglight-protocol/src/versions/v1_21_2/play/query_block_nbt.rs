//! @generated by `froglight-generator` #e606248

use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct QueryBlockNbtPacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: BlockPosition,
}
