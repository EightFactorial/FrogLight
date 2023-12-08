use mc_rs_protocol::Version;

/// A trait for converting between a [`Version`]'s biome ids and their
/// [`ResourceLocation`](mc_rs_core::ResourceLocation) names.
pub trait VersionBiomeIds: Version {
    /// Get the name for the given biome id.
    fn biome_id_to_name(id: &u32) -> Option<&'static str>;
    /// Get the biome id for the given name.
    fn biome_name_to_id(name: &str) -> Option<&'static u32>;
}
