//! @generated by `froglight-generator` #e606248

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlayerMoveFullPacket {
    pub field_0: f64,
    pub field_1: f64,
    pub field_2: f64,
    pub field_3: f32,
    pub field_4: f32,
    pub field_5: u8,
}
