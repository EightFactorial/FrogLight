use mc_rs_protocol::Version;

/// A trait for converting between a [`Version`]'s block ids and their
/// [`ResourceLocation`](mc_rs_core::ResourceLocation) names.
pub trait VersionBlockIds: Version {
    /// Get the name for the given block id.
    fn block_id_to_name(id: &u32) -> Option<&'static str>;
    /// Get the block id for the given name.
    fn block_name_to_id(name: &str) -> Option<&'static u32>;
}
