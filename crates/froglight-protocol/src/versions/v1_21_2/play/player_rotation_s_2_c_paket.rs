//! @generated by `froglight-generator` #e606248

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlayerRotationS2CPaketPacket {
    pub field_0: f32,
    pub field_1: f32,
}
