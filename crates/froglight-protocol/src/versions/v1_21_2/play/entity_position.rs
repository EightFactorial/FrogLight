//! @generated by `froglight-generator` #8ddd9f0

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct EntityPositionPacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: f32,
    pub field_2: f32,
    pub field_3: i32,
    pub field_4: bool,
}
