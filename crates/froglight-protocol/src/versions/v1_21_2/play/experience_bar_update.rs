//! @generated by `froglight-generator` #8ddd9f0

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ExperienceBarUpdatePacket {
    pub field_0: f32,
    #[frog(var)]
    pub field_1: u32,
    #[frog(var)]
    pub field_2: u32,
}
