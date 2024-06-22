use froglight_components::resourcekey::ResourceKey;
use froglight_macros::FrogReadWrite;

use crate::packet::RegistryList;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct DynamicRegistriesPacket {
    pub registry: ResourceKey,
    pub list: RegistryList,
}
