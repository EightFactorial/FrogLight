//! @generated by `froglight-generator` #3ae6f0f

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct OpenHorseScreenPacket {
    pub field_0: u8,
    #[frog(var)]
    pub field_1: u32,
    pub field_2: i32,
}
