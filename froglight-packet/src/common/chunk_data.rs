//! TODO

use alloc::vec::Vec;

#[cfg(feature = "facet")]
use facet_minecraft as mc;
use froglight_biome::prelude::BiomeVersion;
use froglight_block::prelude::BlockVersion;
#[cfg(feature = "std")]
use froglight_world::prelude::Chunk;
use froglight_world::{chunk::ParseError, prelude::NaiveChunk};

use crate::common::unsized_buffer::UnsizedBuffer;

#[cfg(not(feature = "std"))]
pub type ChunkData = RawChunkData;

/// Raw chunk data.
#[cfg(feature = "std")]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet), facet(opaque))]
pub struct ChunkData {
    raw: RawChunkData,
    #[expect(clippy::type_complexity, reason = "function pointer")]
    fn_ptr: fn(&[HeightMapData], &[u8], &[u8], u32, i32) -> Result<Chunk, ParseError>,
}

#[cfg(feature = "std")]
impl PartialEq for ChunkData {
    fn eq(&self, other: &Self) -> bool { self.raw == other.raw }
}

// -------------------------------------------------------------------------------------------------

/// Raw chunk data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct RawChunkData {
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

#[cfg(feature = "std")]
impl ChunkData {
    /// Create a new [`ChunkData`] using the given data and
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    pub const fn new<V: BiomeVersion + BlockVersion>(
        heightmaps: Vec<HeightMapData>,
        block_data: Vec<u8>,
        entity_data: UnsizedBuffer<'static>,
    ) -> Self {
        Self {
            raw: RawChunkData { heightmaps, chunk_data: block_data, entity_data },
            fn_ptr: |_heightmaps, blocks, _entities, height_max, height_min| {
                NaiveChunk::parse_from(blocks, height_max, height_min).map(Chunk::new::<V>)
            },
        }
    }

    /// Get a reference to the inner [`RawChunkData`].
    #[inline]
    #[must_use]
    pub const fn as_raw(&self) -> &RawChunkData { &self.raw }

    /// Attempt to parse the chunk data into a [`Chunk`].
    ///
    /// Requires the maximum and minimum heights of the chunk.
    ///
    /// # Errors
    ///
    /// Returns an error if the chunk data is invalid,
    /// or the height limits are invalid.
    #[inline]
    pub fn parse<V: BiomeVersion + BlockVersion>(
        &self,
        height_max: u32,
        height_min: i32,
    ) -> Result<Chunk, ParseError> {
        match (self.fn_ptr)(
            &self.raw.heightmaps,
            &self.raw.chunk_data,
            &self.raw.entity_data,
            height_max,
            height_min,
        ) {
            Ok(mut chunk) => {
                chunk.convert_into::<V>();
                Ok(chunk)
            }
            Err(err) => Err(err),
        }
    }

    /// Convert this into a [`RawChunkData`].
    ///
    /// Converts the inner chunk data into the provided
    /// [`Version`](froglight_common::version::Version).
    #[inline]
    #[must_use]
    pub fn into_raw<V: BiomeVersion + BlockVersion>(self) -> RawChunkData {
        if let Ok(_chunk) = self.parse::<V>(320, -64) {
            todo!("Write the chunk as RawChunkData")
        } else {
            // Return the raw data if parsing fails
            self.raw
        }
    }

    /// Attempt to create a [`NaiveChunk`] from this chunk data.
    ///
    /// Requires the maximum and minimum heights of the chunk.
    ///
    /// # Errors
    ///
    /// Returns an error if the chunk data is invalid,
    /// or the height limits are invalid.
    #[inline]
    pub fn try_into_naive(
        &self,
        height_max: u32,
        height_min: i32,
    ) -> Result<NaiveChunk, ParseError> {
        (self.fn_ptr)(
            &self.raw.heightmaps,
            &self.raw.chunk_data,
            &self.raw.entity_data,
            height_max,
            height_min,
        )
        .map(Chunk::into_naive)
    }
}

impl RawChunkData {
    /// Create a new [`RawChunkData`] using the given data.
    #[inline]
    #[must_use]
    pub const fn new<V>(
        heightmaps: Vec<HeightMapData>,
        chunk_data: Vec<u8>,
        entity_data: UnsizedBuffer<'static>,
    ) -> Self {
        Self { heightmaps, chunk_data, entity_data }
    }

    /// Convert this into a [`RawChunkData`].
    ///
    /// Does nothing, as this is already a [`RawChunkData`].
    #[inline]
    #[must_use]
    pub const fn into_raw<V>(self) -> Self { self }

    /// Attempt to create a [`NaiveChunk`] from this chunk data.
    ///
    /// Requires the maximum and minimum heights of the chunk.
    ///
    /// # Errors
    ///
    /// Returns an error if the chunk data is invalid,
    /// or the height limits are invalid.
    #[inline]
    pub fn try_into_naive(
        &self,
        height_max: u32,
        height_min: i32,
    ) -> Result<NaiveChunk, ParseError> {
        NaiveChunk::parse_from(&self.chunk_data, height_max, height_min)
    }
}
