//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:level_chunk_with_light"

use crate::common::{chunk_data::ChunkData, unsized_buffer::UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct LevelChunkWithLightS2CPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub chunk_data: ChunkData,
    pub light_data: UnsizedBuffer<'static>,
}
