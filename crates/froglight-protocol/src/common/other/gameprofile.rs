#[cfg(feature = "bevy")]
use bevy_ecs::reflect::ReflectComponent;
#[cfg(feature = "bevy")]
use bevy_reflect::std_traits::ReflectDefault;
use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
use hashbrown::HashMap;
use uuid::Uuid;

/// A player's profile.
///
/// Stores information about a player, like their UUID, name, skin, cape, etc.
#[derive(Debug, Default, Clone, PartialEq, Eq, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component, bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Default, Component))]
pub struct GameProfile {
    /// The player's UUID.
    pub uuid: Uuid,
    /// The player's name.
    pub name: CompactString,
    /// The player's properties.
    pub properties: HashMap<CompactString, ProfileProperty>,
}

/// A property of a player's profile.
///
/// Optionally signed by Mojang.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0])]
pub struct ProfileProperty {
    /// The value of the property.
    pub value: CompactString,
    /// An optional signature.
    pub signature: Option<CompactString>,
}
