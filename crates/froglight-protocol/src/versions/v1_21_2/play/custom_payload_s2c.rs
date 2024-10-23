//! @generated by `froglight-generator` #e606248

use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct CustomPayloadS2CPacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: ResourceKey,
    pub field_2: UnsizedBuffer,
}