use std::{io::Cursor, sync::Arc};

use bevy::{prelude::*, utils::HashMap};
use mc_rs_core::{position::ChunkBlockPos, ResourceLocation};
use mc_rs_protocol::{types::packets::chunk_data::ChunkDataPacket, Version};
use parking_lot::RwLock;

use crate::{
    biomes::traits::VersionBiomeIds,
    blocks::{traits::BlocksTrait, Blocks},
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
    pub fn get_block<V: Version>(&self, pos: ChunkBlockPos) -> Option<Blocks>
    where
        Blocks: BlocksTrait<V>,
    {
        Some(<Blocks as BlocksTrait<V>>::from_u32(
            self.get_block_id(pos)?,
        ))
    }

    /// Get the block id at the given position in the [`Chunk`].
    pub fn get_block_id(&self, mut pos: ChunkBlockPos) -> Option<u32> {
        if pos.x >= Section::SECTION_WIDTH as u8 || pos.z >= Section::SECTION_DEPTH as u8 {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to get block, horizontal position ({}|{}) out of bounds",
                pos.x, pos.z
            );

            return None;
        }

        pos.y -= Self::VERTICAL_SHIFT;
        let Ok(section_index): Result<usize, _> = pos.y.try_into() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to get block, vertical position ({}) out of bounds",
                pos.y
            );

            return None;
        };

        if let Some(section) = self
            .sections
            .read()
            .get(section_index / Chunk::SECTION_COUNT)
        {
            section.blocks.get_data(&pos)
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to get block, section index ({}) out of bounds",
                section_index / Chunk::SECTION_COUNT
            );

            None
        }
    }

    /// Set the block at the given position in the [`Chunk`].
    pub fn set_block<V: Version>(&mut self, block: &Blocks, pos: ChunkBlockPos)
    where
        Blocks: BlocksTrait<V>,
    {
        self.set_block_id(<Blocks as BlocksTrait<V>>::to_u32(block), pos);
    }

    /// Set the block id at the given position in the [`Chunk`].
    ///
    /// # Warning
    /// This function does not check if the block id is valid. You should use
    /// [`Chunk::set_block`](Chunk) instead.
    pub fn set_block_id(&mut self, block_id: u32, mut pos: ChunkBlockPos) {
        if pos.x >= Section::SECTION_WIDTH as u8 || pos.z >= Section::SECTION_DEPTH as u8 {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to set block, horizontal position ({}|{}) out of bounds",
                pos.x, pos.z
            );

            return;
        }

        pos.y -= Self::VERTICAL_SHIFT;
        let Ok(section_index): Result<usize, _> = pos.y.try_into() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to set block, vertical position ({}) out of bounds",
                pos.y
            );

            return;
        };

        if let Some(section) = self
            .sections
            .write()
            .get_mut(section_index / Chunk::SECTION_COUNT)
        {
            section.blocks.set_data(block_id, &pos);
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to set block, section index ({}) out of bounds",
                section_index / Chunk::SECTION_COUNT
            );
        }
    }

    /// Get the biome at the given position in the [`Chunk`].
    pub fn get_biome<V: VersionBiomeIds>(&self, pos: ChunkBlockPos) -> Option<ResourceLocation>
    where
        Blocks: BlocksTrait<V>,
    {
        if let Some(biome_id) = self.get_biome_id(pos) {
            if let Some(biome) = V::biome_id_to_name(&biome_id) {
                Some(ResourceLocation::new(biome))
            } else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Failed to get biome name, invalid biome id");

                None
            }
        } else {
            None
        }
    }

    /// Get the biome id at the given position in the [`Chunk`].
    pub fn get_biome_id(&self, mut pos: ChunkBlockPos) -> Option<u32> {
        if pos.x >= Section::SECTION_WIDTH as u8 || pos.z >= Section::SECTION_DEPTH as u8 {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to get biome, horizontal position ({}|{}) out of bounds",
                pos.x, pos.z
            );

            return None;
        }

        pos.y -= Self::VERTICAL_SHIFT;
        let Ok(section_index): Result<usize, _> = pos.y.try_into() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to get biome, vertical position ({}) out of bounds",
                pos.y
            );

            return None;
        };

        if let Some(section) = self
            .sections
            .read()
            .get(section_index / Chunk::SECTION_COUNT)
        {
            let id = section.biomes.get_data(&pos);

            #[cfg(any(debug_assertions, feature = "debug"))]
            if id.is_none() {
                error!("Failed to get biome id from section");
            }

            id
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to get biome, section index ({}) out of bounds",
                section_index / Chunk::SECTION_COUNT
            );

            None
        }
    }

    /// Set the biome at the given position in the [`Chunk`].
    pub fn set_biome<V: VersionBiomeIds>(&mut self, biome: &ResourceLocation, pos: ChunkBlockPos) {
        if let Some(biome) = V::biome_name_to_id(biome) {
            self.set_biome_id(*biome, pos);
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to set biome, invalid biome name");
        }
    }

    /// Set the biome id at the given position in the [`Chunk`].
    ///
    /// # Warning
    /// This function does not check if the biome id is valid. You should use
    /// [`Chunk::set_biome`](Chunk) instead.
    pub fn set_biome_id(&mut self, biome_id: u32, mut pos: ChunkBlockPos) {
        if pos.x >= Section::SECTION_WIDTH as u8 || pos.z >= Section::SECTION_DEPTH as u8 {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to set biome, horizontal position ({}|{}) out of bounds",
                pos.x, pos.z
            );

            return;
        }

        pos.y -= Self::VERTICAL_SHIFT;
        let Ok(section_index): Result<usize, _> = pos.y.try_into() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to set biome, vertical position ({}) out of bounds",
                pos.y
            );

            return;
        };

        if let Some(section) = self
            .sections
            .write()
            .get_mut(section_index / Chunk::SECTION_COUNT)
        {
            section.biomes.set_data(biome_id, &pos);
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to set biome, section index ({}) out of bounds",
                section_index / Chunk::SECTION_COUNT
            );
        }
    }
}
