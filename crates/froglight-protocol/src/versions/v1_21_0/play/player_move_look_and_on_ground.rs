//! @generated by `froglight-generator` #73eaa37

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlayerMoveLookAndOnGroundPacket {
    pub field_0: f32,
    pub field_1: f32,
    pub field_2: u8,
}
