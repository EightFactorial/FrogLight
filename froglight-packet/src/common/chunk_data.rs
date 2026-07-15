//! TODO

use alloc::vec::Vec;
use core::fmt::Debug;

use froglight_biome::prelude::BiomeVersion;
use froglight_block::prelude::BlockVersion;
#[cfg(feature = "facet")]
use froglight_facet as mc;
use froglight_nbt::types::indexed::alloc::IndexedNbtCow;
use froglight_world::{
    component::ChunkBlockPos,
    naive::ParseError,
    prelude::{Chunk, NaiveChunk},
};

/// Raw chunk data.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct RawChunkData {
    /// The chunk's height maps.
    pub heightmaps: Vec<RawHeightMapData>,
    /// The chunk's block data.
    pub chunk_data: Vec<u8>,
    /// The chunk's block entities.
    pub entity_data: Vec<RawEntityData>,
}

/// Raw height map data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct RawHeightMapData {
    /// The type of height map.
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub kind: u32,
    /// The height map data.
    pub data: Vec<u64>,
}

/// Raw entity data.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct RawEntityData {
    /// The position of the block entity.
    #[cfg_attr(feature = "facet", facet(mc::with = ChunkBlockPos::WITH_PACKED))]
    pub position: ChunkBlockPos,
    /// The type of block entity.
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub kind: u32,
    /// The block entity's NBT data.
    pub nbt: IndexedNbtCow<'static>,
}

impl RawChunkData {
    /// Attempt to parse a [`Chunk`] from [`RawChunkData`].
    ///
    /// # Errors
    ///
    /// Returns an error if the data is invalid,
    /// or if `height_max` and `height_min` are incorrect.
    #[inline]
    pub fn try_parse<V: BiomeVersion + BlockVersion>(
        &self,
        height_max: u32,
        height_min: i32,
    ) -> Result<Chunk, ParseError> {
        self.try_parse_naive(height_max, height_min).map(Chunk::new::<V>)
    }

    /// Attempt to parse a [`NaiveChunk`] from [`RawChunkData`].
    ///
    /// # Errors
    ///
    /// Returns an error if the data is invalid,
    /// or if `height_max` and `height_min` are incorrect.
    pub fn try_parse_naive(
        &self,
        height_max: u32,
        height_min: i32,
    ) -> Result<NaiveChunk, ParseError> {
        NaiveChunk::parse_from(&self.chunk_data, height_max, height_min)
    }
}
