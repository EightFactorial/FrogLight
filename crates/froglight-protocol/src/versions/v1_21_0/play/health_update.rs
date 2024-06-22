//! @generated by `froglight-generator` #73eaa37

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct HealthUpdatePacket {
    pub field_0: f32,
    #[frog(var)]
    pub field_1: u32,
    pub field_2: f32,
}
