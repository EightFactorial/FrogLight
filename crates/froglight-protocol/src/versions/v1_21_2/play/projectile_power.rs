//! @generated by `froglight-generator` #8ddd9f0

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ProjectilePowerPacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: f64,
}
