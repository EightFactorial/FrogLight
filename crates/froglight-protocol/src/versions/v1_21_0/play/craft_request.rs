//! @generated by `froglight-generator` #73eaa37

use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct CraftRequestPacket {
    pub field_0: u8,
    pub field_1: ResourceKey,
    pub field_2: bool,
}