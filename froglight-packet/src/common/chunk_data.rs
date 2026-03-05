//! TODO

use alloc::vec::Vec;
use core::fmt::Debug;

#[cfg(feature = "facet")]
use facet_minecraft as mc;
#[cfg(feature = "std")]
use froglight_biome::prelude::BiomeVersion;
#[cfg(feature = "std")]
use froglight_block::prelude::BlockVersion;
use froglight_world::{
    chunk::ParseError,
    prelude::{Chunk, NaiveChunk},
};

use crate::common::unsized_buffer::UnsizedBuffer;

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

/// Chunk data.
///
/// Provides methods for accessing both raw and parsed chunk data
/// and converting them between
/// [`Version`](froglight_common::version::Version)s.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct ChunkData(ChunkDataInner);

#[repr(u8)]
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet), facet(opaque))]
enum ChunkDataInner {
    Chunk(Chunk),
    Versioned {
        height_max_min: Option<(u32, i32)>,
        raw_data: RawChunkData,
        fn_ptr: fn(&RawChunkData, u32, i32) -> Result<Chunk, ParseError>,
    },
    Unversioned {
        height_max_min: Option<(u32, i32)>,
        raw_data: RawChunkData,
        fn_ptr: fn(&RawChunkData, u32, i32) -> Result<NaiveChunk, ParseError>,
    },
}

impl ChunkData {
    /// Create a new [`ChunkData`] from a [`Chunk`].
    #[inline]
    #[must_use]
    pub const fn new(chunk: Chunk) -> Self { Self(ChunkDataInner::Chunk(chunk)) }

    /// Create a new [`ChunkData`] from [`RawChunkData`].
    #[must_use]
    #[cfg(feature = "std")]
    pub const fn new_versioned<V: BiomeVersion + BlockVersion>(raw_data: RawChunkData) -> Self {
        Self::new_versioned_with(raw_data, None, |raw, max, min| {
            NaiveChunk::parse_from(&raw.chunk_data, max, min).map(Chunk::new::<V>)
        })
    }

    /// Create a new [`ChunkData`] from [`RawChunkData`] and a parsing function.
    ///
    /// Optionally accepts the chunk's maximum and minimum heights to use while
    /// parsing.
    #[must_use]
    pub const fn new_versioned_with(
        raw_data: RawChunkData,
        height_max_min: Option<(u32, i32)>,
        fn_ptr: fn(&RawChunkData, u32, i32) -> Result<Chunk, ParseError>,
    ) -> Self {
        Self(ChunkDataInner::Versioned { height_max_min, raw_data, fn_ptr })
    }

    /// Create a new [`ChunkData`] from [`RawChunkData`] without version
    /// information.
    #[must_use]
    pub const fn new_unversioned(raw_data: RawChunkData) -> Self {
        Self::new_unversioned_with(raw_data, None, |raw, max, min| {
            NaiveChunk::parse_from(&raw.chunk_data, max, min)
        })
    }

    /// Create a new [`ChunkData`] from [`RawChunkData`] without version
    /// information and a parsing function.
    ///
    /// Optionally accepts the chunk's maximum and minimum heights to use while
    /// parsing.
    #[must_use]
    pub const fn new_unversioned_with(
        raw_data: RawChunkData,
        height_max_min: Option<(u32, i32)>,
        fn_ptr: fn(&RawChunkData, u32, i32) -> Result<NaiveChunk, ParseError>,
    ) -> Self {
        Self(ChunkDataInner::Unversioned { height_max_min, raw_data, fn_ptr })
    }

    /// Get the chunk data as a [`Chunk`], parsing it if necessary.
    ///
    /// Automatically converts the chunk into the specified
    /// [`Version`](froglight_common::version::Version) after parsing.
    ///
    /// Optionally accepts the chunk's maximum and minimum heights to use while
    /// parsing.
    ///
    /// # Errors
    ///
    /// Returns an error if the chunk data is invalid or the heights are
    /// incorrect.
    #[cfg(feature = "std")]
    pub fn as_chunk<V: BiomeVersion + BlockVersion>(
        &self,
        height_max_min: Option<(u32, i32)>,
    ) -> Result<Chunk, ParseError> {
        match &self.0 {
            ChunkDataInner::Chunk(chunk) => {
                let mut chunk = chunk.clone();
                chunk.convert_into::<V>();
                Ok(chunk)
            }
            ChunkDataInner::Versioned { height_max_min: raw_max_min, raw_data, fn_ptr } => {
                match (height_max_min, raw_max_min) {
                    // Use the provided height max and min if both are provided.
                    (Some((max, min)), _) => {
                        let mut chunk = fn_ptr(raw_data, max, min)?;
                        chunk.convert_into::<V>();
                        Ok(chunk)
                    }
                    // Use the height max and min from the raw data if available.
                    (None, Some((max, min))) => {
                        let mut chunk = fn_ptr(raw_data, *max, *min)?;
                        chunk.convert_into::<V>();
                        Ok(chunk)
                    }
                    // Otherwise attempt to guess using common values.
                    (None, None) => {
                        for (max, min) in [(320, -64), (256, 0)] {
                            if let Ok(mut chunk) = fn_ptr(raw_data, max, min) {
                                chunk.convert_into::<V>();
                                return Ok(chunk);
                            }
                        }
                        todo!()
                    }
                }
            }
            ChunkDataInner::Unversioned { height_max_min: raw_max_min, raw_data, fn_ptr } => {
                match (height_max_min, raw_max_min) {
                    // Use the provided height max and min if both are provided.
                    (Some((max, min)), _) => {
                        let chunk = fn_ptr(raw_data, max, min)?;
                        Ok(Chunk::new::<V>(chunk))
                    }
                    // Use the height max and min from the raw data if available.
                    (None, Some((max, min))) => {
                        let chunk = fn_ptr(raw_data, *max, *min)?;
                        Ok(Chunk::new::<V>(chunk))
                    }
                    // Otherwise attempt to guess using common values.
                    (None, None) => {
                        for (max, min) in [(320, -64), (256, 0)] {
                            if let Ok(chunk) = fn_ptr(raw_data, max, min) {
                                return Ok(Chunk::new::<V>(chunk));
                            }
                        }
                        todo!()
                    }
                }
            }
        }
    }

    /// Get the chunk data as a [`Chunk`], parsing it if necessary.
    ///
    /// Optionally accepts the chunk's maximum and minimum heights to use while
    /// parsing.
    ///
    /// # Errors
    ///
    /// Returns an error if the chunk data is invalid or the heights are
    /// incorrect.
    pub fn as_naive(&self, height_max_min: Option<(u32, i32)>) -> Result<NaiveChunk, ParseError> {
        match &self.0 {
            ChunkDataInner::Chunk(chunk) => Ok(chunk.clone().into_naive()),
            ChunkDataInner::Versioned { height_max_min: raw_max_min, raw_data, fn_ptr } => {
                match (height_max_min, raw_max_min) {
                    // Use the provided height max and min if both are provided.
                    (Some((max, min)), _) => fn_ptr(raw_data, max, min).map(Chunk::into_naive),
                    // Use the height max and min from the raw data if available.
                    (None, Some((max, min))) => fn_ptr(raw_data, *max, *min).map(Chunk::into_naive),
                    // Otherwise attempt to guess using common values.
                    (None, None) => {
                        for (max, min) in [(320, -64), (256, 0)] {
                            if let Ok(chunk) = fn_ptr(raw_data, max, min) {
                                return Ok(chunk.into_naive());
                            }
                        }
                        todo!()
                    }
                }
            }
            ChunkDataInner::Unversioned { height_max_min: raw_max_min, raw_data, fn_ptr } => {
                match (height_max_min, raw_max_min) {
                    // Use the provided height max and min if both are provided.
                    (Some((max, min)), _) => fn_ptr(raw_data, max, min),
                    // Use the height max and min from the raw data if available.
                    (None, Some((max, min))) => fn_ptr(raw_data, *max, *min),
                    // Otherwise attempt to guess using common values.
                    (None, None) => {
                        for (max, min) in [(320, -64), (256, 0)] {
                            if let Ok(chunk) = fn_ptr(raw_data, max, min) {
                                return Ok(chunk);
                            }
                        }
                        todo!()
                    }
                }
            }
        }
    }

    /// Get the chunk data as [`RawChunkData`], writing it if necessary.
    ///
    /// Automatically converts the chunk into the specified
    /// [`Version`](froglight_common::version::Version) before writing.
    #[must_use]
    #[cfg(feature = "std")]
    pub fn as_raw<V: BiomeVersion + BlockVersion>(&self) -> RawChunkData {
        match &self.0 {
            ChunkDataInner::Chunk(chunk) => {
                let mut chunk = chunk.clone();
                chunk.convert_into::<V>();

                todo!("Write the chunk as RawChunkData")
            }
            ChunkDataInner::Versioned { raw_data, .. }
            | ChunkDataInner::Unversioned { raw_data, .. } => raw_data.clone(),
        }
    }

    /// Get the chunk data as [`RawChunkData`], writing it if necessary.
    #[must_use]
    pub fn as_raw_original(&self) -> RawChunkData {
        match &self.0 {
            ChunkDataInner::Chunk(_chunk) => todo!("Write the chunk as RawChunkData"),
            ChunkDataInner::Versioned { raw_data, .. }
            | ChunkDataInner::Unversioned { raw_data, .. } => raw_data.clone(),
        }
    }
}

impl Debug for ChunkDataInner {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Chunk(..) => f.debug_tuple("Chunk").finish_non_exhaustive(),
            Self::Versioned { .. } => f.debug_tuple("Versioned").finish_non_exhaustive(),
            Self::Unversioned { .. } => f.debug_tuple("Unversioned").finish_non_exhaustive(),
        }
    }
}

impl Eq for ChunkDataInner {}
impl PartialEq for ChunkDataInner {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ChunkDataInner::Chunk(chunk_a), ChunkDataInner::Chunk(chunk_b)) => chunk_a == chunk_b,
            (
                ChunkDataInner::Versioned { raw_data: raw_a, .. }
                | ChunkDataInner::Unversioned { raw_data: raw_a, .. },
                ChunkDataInner::Versioned { raw_data: raw_b, .. }
                | ChunkDataInner::Unversioned { raw_data: raw_b, .. },
            ) => raw_a == raw_b,
            _ => false,
        }
    }
}
