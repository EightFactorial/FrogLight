//! @generated by `froglight-generator` #73eaa37

use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct AdvancementTabPacket {
    pub field_0: Enum,
    pub field_1: ResourceKey,
}
