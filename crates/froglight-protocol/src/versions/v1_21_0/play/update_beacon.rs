//! @generated by `froglight-generator` #3ae6f0f

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct UpdateBeaconPacket {
    #[frog(var)]
    pub field_0: u32,
    #[frog(var)]
    pub field_1: u32,
}
