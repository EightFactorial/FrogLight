use froglight_common::ResourceKey;
use froglight_macros::FrogReadWrite;

use crate::packet::RegistryData;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct DynamicRegistriesPacket {
    pub identifier: ResourceKey,
    pub registry_data: Vec<RegistryData>,
}
