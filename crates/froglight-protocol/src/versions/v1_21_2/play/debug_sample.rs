//! @generated by `froglight-generator` #e606248

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct DebugSamplePacket {
    pub field_0: Vec<i64>,
    pub field_1: Enum,
}
