use froglight_components::resourcekey::ResourceKey;
use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

/// A list of registries.
#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct RegistryList(pub Vec<RegistryData>);

/// Data about a registry.
#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct RegistryData {
    /// The name of the registry.
    pub identifier: ResourceKey,
    /// Optional data about the registry.
    pub data: Option<Nbt>,
}
