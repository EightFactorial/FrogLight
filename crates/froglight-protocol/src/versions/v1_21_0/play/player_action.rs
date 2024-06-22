//! @generated by `froglight-generator` #3ae6f0f

use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlayerActionPacket {
    pub field_0: Enum,
    pub field_1: BlockPosition,
    pub field_2: u8,
    #[frog(var)]
    pub field_3: u32,
}
