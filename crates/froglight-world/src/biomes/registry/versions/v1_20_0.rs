use froglight_core::common::ResourceKey;
use froglight_macros::frog_version_biomes;
use froglight_protocol::versions::v1_20_0::V1_20_0;

use crate::biomes::{biome_list::BiomePlains, traits::BiomeRegistration, BiomeType};

frog_version_biomes! {
    V1_20_0,
    Plains,
}

impl BiomeType<V1_20_0> for BiomePlains {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:plains") }
}
