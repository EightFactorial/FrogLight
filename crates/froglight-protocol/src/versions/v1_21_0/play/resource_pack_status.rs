//! @generated by `froglight-generator` #ecfea09

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct ResourcePackStatusPacket {
    pub field_0: Uuid,
    pub field_1: Enum,
}
