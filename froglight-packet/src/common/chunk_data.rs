//! TODO

use alloc::vec::Vec;

#[cfg(feature = "facet")]
use facet_minecraft as mc;
use froglight_world::{chunk::ParseError, prelude::NaiveChunk};

use crate::common::unsized_buffer::UnsizedBuffer;

/// Raw chunk data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct ChunkData {
    /// The chunk's height maps.
    pub heightmaps: Vec<HeightMapData>,
    /// The chunk's block data.
    pub chunk_data: Vec<u8>,
    /// The chunk's block entities.
    pub entity_data: UnsizedBuffer<'static>,
}

/// Raw height map data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct HeightMapData {
    /// The type of height map.
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub kind: u32,
    /// The height map data.
    pub data: Vec<u64>,
}

// -------------------------------------------------------------------------------------------------

impl ChunkData {
    /// Attempt to create a [`NaiveChunk`] from this chunk data.
    ///
    /// Requires the maximum and minimum heights of the chunk.
    ///
    /// # Errors
    ///
    /// Returns an error if the chunk data is invalid.
    pub fn try_into_naive(
        &self,
        height_max: u32,
        height_min: i32,
    ) -> Result<NaiveChunk, ParseError> {
        NaiveChunk::try_from(&self.chunk_data, height_max, height_min)
    }
}
