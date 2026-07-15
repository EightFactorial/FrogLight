//! TODO

use alloc::vec::Vec;

/// Raw light data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct RawLightData {
    /// A bitset mask indicating which sections have sky light.
    pub sky_light_mask: Vec<u64>,
    /// A bitset mask indicating which sections have block light.
    pub block_light_mask: Vec<u64>,
    /// A bitset mask indicating which sections have no sky light.
    pub empty_sky_light_mask: Vec<u64>,
    /// A bitset mask indicating which sections have no block light.
    pub empty_block_light_mask: Vec<u64>,
    /// The sky light data for each section.
    pub sky_light_array: Vec<Vec<u8>>,
    /// The block light data for each section.
    pub block_light_array: Vec<Vec<u8>>,
}
