use mc_rs_protocol::Version;

/// A trait that implements a conversion from a version's block states and biomes to a global
/// palette.
pub trait GlobalPalette: Version {
    fn to_global_block(state_id: u32) -> u32;

    fn batch_to_global_block(state_ids: impl IntoIterator<Item = u32>) -> Vec<u32> {
        state_ids.into_iter().map(Self::to_global_block).collect()
    }

    fn to_global_biome(biome_id: u32) -> u32;

    fn batch_to_global_biome(biome_ids: impl IntoIterator<Item = u32>) -> Vec<u32> {
        biome_ids.into_iter().map(Self::to_global_biome).collect()
    }
}
