//! @generated by `froglight-generator` #3ae6f0f

use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct NbtQueryResponsePacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: Nbt,
}
