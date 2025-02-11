//! @generated by `froglight-generator` #8ddd9f0

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlaySoundPacket {
    pub field_0: Enum,
    pub field_1: i32,
    pub field_2: i32,
    pub field_3: i32,
    pub field_4: f32,
    pub field_5: f32,
    pub field_6: i64,
}
