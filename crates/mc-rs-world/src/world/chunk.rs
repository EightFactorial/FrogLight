use std::{io::Cursor, sync::Arc};

use bevy::{prelude::*, utils::HashMap};
use mc_rs_core::{position::ChunkBlockPos, ResourceLocation};
use mc_rs_protocol::types::packets::chunk_data::ChunkDataPacket;
use parking_lot::RwLock;

use crate::{
    biomes::traits::VersionBiomeIds, blocks::traits::VersionBlockIds,
    world::tasks::ChunkDecodeError,
};

use super::{heightmap::HeightMapType, section::Section, tasks::DecodeResult, HeightMap};

#[derive(Debug, Clone, Component)]
pub struct Chunk {
    pub sections: Arc<RwLock<[Section; Self::SECTION_COUNT]>>,
    pub heightmaps: HashMap<HeightMapType, HeightMap>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            sections: Arc::new(RwLock::new(
                vec![Section::default(); Self::SECTION_COUNT]
                    .try_into()
                    .unwrap(),
            )),
            heightmaps: HashMap::new(),
        }
    }
}

impl Chunk {
    pub const WORLD_HEIGHT: usize = 384;
    pub const VERTICAL_SHIFT: i32 = -64;

    pub const SECTION_COUNT: usize = Self::WORLD_HEIGHT / Section::SECTION_HEIGHT;

    pub(super) async fn decode_chunk(data: ChunkDataPacket) -> DecodeResult {
        let mut sections = Vec::with_capacity(Self::SECTION_COUNT);

        let mut section_data = Cursor::new(data.data.as_slice());
        for _ in 0..Self::SECTION_COUNT {
            sections.push(Section::decode_section(&mut section_data).await?);
        }

        let Ok(sections) = sections.try_into() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to convert Sections Vec into Array");

            return Err(ChunkDecodeError::InvalidSection);
        };

        // TODO: Decode heightmap NBT data
        let heightmaps = HashMap::new();

        Ok(Self {
            sections: Arc::new(RwLock::new(sections)),
            heightmaps,
        })
    }

    /// Get the block at the given position in the [`Chunk`].
    pub fn get_block<V: VersionBlockIds>(
        &self,
        mut pos: ChunkBlockPos,
    ) -> Option<ResourceLocation> {
        if pos.x >= Section::SECTION_WIDTH as u8 || pos.z >= Section::SECTION_DEPTH as u8 {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to get block, horizontal position out of bounds");

            return None;
        }

        pos.y -= Self::VERTICAL_SHIFT;
        let Ok(section_index): Result<usize, _> = pos.y.try_into() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to get block, vertical position out of bounds");

            return None;
        };

        if let Some(section) = self.sections.read().get(section_index) {
            let block_id = section.blocks.get_data(&pos);

            let Some(block_id) = block_id else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Failed to get block");

                return None;
            };

            if let Some(block) = V::block_id_to_name(&block_id) {
                Some(ResourceLocation::new(block))
            } else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Failed to get block, invalid block id");

                None
            }
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to get block, section index out of bounds");

            None
        }
    }

    /// Set the block at the given position in the [`Chunk`].
    pub fn set_block<V: VersionBlockIds>(
        &mut self,
        block: &ResourceLocation,
        mut pos: ChunkBlockPos,
    ) {
        if pos.x >= Section::SECTION_WIDTH as u8 || pos.z >= Section::SECTION_DEPTH as u8 {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to set block, horizontal position out of bounds");

            return;
        }

        pos.y -= Self::VERTICAL_SHIFT;
        let Ok(section_index): Result<usize, _> = pos.y.try_into() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to set block, vertical position out of bounds");

            return;
        };

        let Some(block_id) = V::block_name_to_id(block.as_str()) else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to set block, invalid block name");

            return;
        };

        if let Some(section) = self.sections.write().get_mut(section_index) {
            section.blocks.set_data(*block_id, &pos);
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to set block, section index out of bounds");
        }
    }

    /// Get the biome at the given position in the [`Chunk`].
    pub fn get_biome<V: VersionBiomeIds>(
        &self,
        mut pos: ChunkBlockPos,
    ) -> Option<ResourceLocation> {
        if pos.x >= Section::SECTION_WIDTH as u8 || pos.z >= Section::SECTION_DEPTH as u8 {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to get biome, horizontal position out of bounds");

            return None;
        }

        pos.y -= Self::VERTICAL_SHIFT;
        let Ok(section_index): Result<usize, _> = pos.y.try_into() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to get biome, vertical position out of bounds");

            return None;
        };

        if let Some(section) = self.sections.read().get(section_index) {
            let biome_id = section.biomes.get_data(&pos);

            let Some(biome_id) = biome_id else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Failed to get biome");

                return None;
            };

            if let Some(biome) = V::biome_id_to_name(&biome_id) {
                Some(ResourceLocation::new(biome))
            } else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Failed to get biome, invalid biome id");

                None
            }
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to get biome, section index out of bounds");

            None
        }
    }

    /// Set the biome at the given position in the [`Chunk`].
    pub fn set_biome<V: VersionBiomeIds>(
        &mut self,
        biome: &ResourceLocation,
        mut pos: ChunkBlockPos,
    ) {
        if pos.x >= Section::SECTION_WIDTH as u8 || pos.z >= Section::SECTION_DEPTH as u8 {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to set biome, horizontal position out of bounds");

            return;
        }

        pos.y -= Self::VERTICAL_SHIFT;
        let Ok(section_index): Result<usize, _> = pos.y.try_into() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to set biome, vertical position out of bounds");

            return;
        };

        let Some(biome_id) = V::biome_name_to_id(biome.as_str()) else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to set biome, invalid biome name");

            return;
        };

        if let Some(section) = self.sections.write().get_mut(section_index) {
            section.biomes.set_data(*biome_id, &pos);
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to set biome, section index out of bounds");
        }
    }
}

// #[test]
// fn get_block() {}
