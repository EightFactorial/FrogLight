//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:level_chunk_with_light"

use crate::common::{chunk_data::RawChunkData, light_data::RawLightData};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct LevelChunkWithLightS2CPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub chunk_data: RawChunkData,
    pub light_data: RawLightData,
}
