//! @generated by `froglight-generator` #ecfea09

use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct PlayerActionPacket {
    pub field_0: Enum,
    pub field_1: BlockPosition,
    pub field_2: u8,
    #[frog(var)]
    pub field_3: u32,
}
