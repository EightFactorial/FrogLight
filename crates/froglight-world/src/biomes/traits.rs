use std::fmt::Debug;

use bevy_reflect::Reflect;
use froglight_core::common::ResourceKey;
use froglight_protocol::traits::Version;

use crate::biomes::{registry::InnerBiomeRegistry, BiomeEnum};

/// A biome.
pub trait BiomeType<V: Version>: Debug + Reflect {
    /// Get the resource key of the biome.
    fn resource_key(&self) -> ResourceKey;
}

/// Extra methods for biomes.
pub trait BiomeExt<V: Version>: Sized + BiomeType<V> + Default {}

/// A trait that registers biomes inside the biome registry.
pub trait BiomeRegistration: Version {
    type Biomes: BiomeEnumTrait<Self> + Into<BiomeEnum>;

    /// Register the default biomes.
    fn register_default(registry: &mut InnerBiomeRegistry<Self>);
}

/// A collection of biomes specific to a version.
#[allow(unreachable_pub)]
pub trait BiomeEnumTrait<V: BiomeRegistration>: Sized + Debug + Reflect {
    /// Get the biome using a biome id and the biome registry.
    #[must_use]
    fn get_biome(id: u32, registry: &InnerBiomeRegistry<V>) -> Option<Self>;
}
