//! @generated by `froglight-generator` #3ae6f0f

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct BoatPaddleStatePacket {
    pub field_0: bool,
    pub field_1: bool,
}
