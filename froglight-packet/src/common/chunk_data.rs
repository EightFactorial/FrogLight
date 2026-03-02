//! TODO

use alloc::{borrow::Cow, vec::Vec};

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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct ChunkData {
    /// The chunk's height maps.
    pub heightmaps: Vec<HeightMapData>,
    /// The chunk's block data.
    pub chunk_data: ChunkDataWrapper<'static>,
    /// The chunk's block entities.
    pub entity_data: UnsizedBuffer<'static>,
}

/// A chunk's block data and a parsing function.
#[cfg(feature = "std")]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet), facet(opaque))]
pub struct ChunkDataWrapper<'a> {
    data: Cow<'a, [u8]>,
    fn_ptr: fn(&[u8], u32, i32) -> Result<Chunk, ParseError>,
}

#[cfg(feature = "std")]
impl<'a> ChunkDataWrapper<'a> {
    /// Create a new [`ChunkDataWrapper`] using the given data and
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    pub const fn wrap_borrowed<V: BiomeVersion + BlockVersion>(data: &'a [u8]) -> Self {
        Self::wrap_using(Cow::Borrowed(data), Self::default_parser::<V>)
    }

    /// Create a new [`ChunkDataWrapper`] using the given data and
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    pub const fn wrap<V: BiomeVersion + BlockVersion>(data: Vec<u8>) -> Self {
        Self::wrap_using(Cow::Owned(data), Self::default_parser::<V>)
    }

    /// The default [`Version`](froglight_common::version::Version)'ed [`Chunk`]
    /// parser.
    fn default_parser<V: BiomeVersion + BlockVersion>(
        data: &[u8],
        height_max: u32,
        height_min: i32,
    ) -> Result<Chunk, ParseError> {
        NaiveChunk::try_from(data, height_max, height_min).map(Chunk::new::<V>)
    }

    /// Create a new [`ChunkDataWrapper`] using the given data and fn.
    #[inline]
    #[must_use]
    pub const fn wrap_using(
        data: Cow<'a, [u8]>,
        fn_ptr: fn(&[u8], u32, i32) -> Result<Chunk, ParseError>,
    ) -> ChunkDataWrapper<'a> {
        ChunkDataWrapper { data, fn_ptr }
    }

    /// Attempt to parse a [`Chunk`] using the provided heights.
    ///
    /// # Errors
    ///
    /// Returns an error if the data was invalid or the heights are incorrect.
    #[inline]
    pub fn parse(&self, height_max: u32, height_min: i32) -> Result<Chunk, ParseError> {
        (self.fn_ptr)(&self.data, height_max, height_min)
    }

    /// Return the inner data.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Cow<'a, [u8]> { self.data }
}

#[cfg(feature = "std")]
impl PartialEq for ChunkDataWrapper<'_> {
    fn eq(&self, other: &Self) -> bool { self.data == other.data }
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
        chunk_data: Vec<u8>,
        entity_data: UnsizedBuffer<'static>,
    ) -> Self {
        Self { heightmaps, chunk_data: ChunkDataWrapper::wrap::<V>(chunk_data), entity_data }
    }

    /// Convert this into a [`RawChunkData`].
    ///
    /// Converts the inner chunk data into the provided
    /// [`Version`](froglight_common::version::Version).
    #[inline]
    #[must_use]
    pub fn into_raw<V: BiomeVersion + BlockVersion>(self) -> RawChunkData {
        // Parse the chunk data, or return the data as-is if it was invalid.
        let Ok(chunk) = self.chunk_data.parse(320, -64) else {
            return RawChunkData {
                heightmaps: self.heightmaps,
                chunk_data: self.chunk_data.into_inner().into_owned(),
                entity_data: self.entity_data,
            };
        };

        // Convert the chunk data into the provided version.
        let _chunk = chunk.convert_into::<V>();

        todo!("Write the chunk as RawChunkData")
    }
}

impl RawChunkData {
    /// Create a new [`RawChunkData`] using the given data.
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
    /// Returns an error if the chunk data is invalid.
    pub fn try_into_naive(
        &self,
        height_max: u32,
        height_min: i32,
    ) -> Result<NaiveChunk, ParseError> {
        NaiveChunk::try_from(&self.chunk_data, height_max, height_min)
    }
}
