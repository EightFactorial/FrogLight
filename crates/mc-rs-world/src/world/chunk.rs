use std::{io::Cursor, sync::Arc};

use bevy::{prelude::*, utils::HashMap};
use mc_rs_core::position::ChunkBlockPos;
use mc_rs_protocol::types::packets::chunk_data::ChunkDataPacket;
use parking_lot::RwLock;

use crate::world::tasks::ChunkDecodeError;

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
    pub fn get_block(&self, mut pos: ChunkBlockPos) -> Option<u32> {
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
            section.blocks.get_data(&pos)
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to get block, section index out of bounds");

            None
        }
    }

    /// Set the block at the given position in the [`Chunk`].
    pub fn set_block(&mut self, data: u32, mut pos: ChunkBlockPos) {
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

        if let Some(section) = self.sections.write().get_mut(section_index) {
            section.blocks.set_data(data, &pos);
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to set block, section index out of bounds");
        }
    }

    /// Get the biome at the given position in the [`Chunk`].
    pub fn get_biome(&self, mut pos: ChunkBlockPos) -> Option<u32> {
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
            section.biomes.get_data(&pos)
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to get biome, section index out of bounds");

            None
        }
    }

    /// Set the biome at the given position in the [`Chunk`].
    pub fn set_biome(&mut self, data: u32, mut pos: ChunkBlockPos) {
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

        if let Some(section) = self.sections.write().get_mut(section_index) {
            section.biomes.set_data(data, &pos);
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to set biome, section index out of bounds");
        }
    }
}
