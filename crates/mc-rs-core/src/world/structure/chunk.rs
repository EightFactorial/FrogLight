use std::{io::Cursor, sync::Arc};

use azalea_nbt::Nbt;
use bevy::prelude::*;
use mc_rs_protocol::{
    buffer::DecodeError,
    types::{
        packets::chunk_data::{ChunkDataPacket, SectionDataPacket},
        position::{ChunkPos, ChunkSectionPos},
    },
};
use parking_lot::RwLock;

use crate::world::{
    palette::GlobalPalette, WorldType, CHUNK_SIZE, CHUNK_VERT_DISPLACEMENT, SECTION_COUNT,
};

use super::section::Section;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct ChunkEntity(pub Entity);

pub type ChunkSections = Arc<RwLock<[Section; SECTION_COUNT]>>;

#[derive(Debug, Clone, Component)]
pub struct Chunk {
    pub sections: ChunkSections,
    pub motion_blocking: Vec<i64>,
    pub world_surface: Vec<i64>,
    pub world_type: WorldType,
    pub position: ChunkPos,
}

impl Chunk {
    /// Decodes a chunk from a chunk data packet.
    pub fn decode<V: GlobalPalette>(
        position: ChunkPos,
        world_type: WorldType,
        chunk_data: ChunkDataPacket,
        world: &mut bevy::ecs::world::World,
    ) -> Result<ChunkEntity, DecodeError> {
        let mut cursor = Cursor::new(chunk_data.data.as_slice());

        let mut sections = Vec::with_capacity(SECTION_COUNT);
        for _ in 0..SECTION_COUNT {
            sections.push(Section::decode::<V>(&mut cursor)?);
        }

        let (motion_blocking, world_surface) = Self::unwrap_heightmaps(chunk_data.heightmaps)
            .unwrap_or_else(|| {
                log::warn!("Failed to unwrap chunk heightmaps");
                Default::default()
            });

        let visibility = if sections.iter().all(|section| section.block_count == 0) {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };

        let entity = world.spawn((
            Self {
                sections: Arc::new(RwLock::new(sections.try_into().unwrap())),
                motion_blocking,
                world_surface,
                world_type,
                position,
            },
            TransformBundle {
                local: Transform::from_xyz(
                    (position.x * CHUNK_SIZE as i32) as f32,
                    CHUNK_VERT_DISPLACEMENT as f32,
                    (position.y * CHUNK_SIZE as i32) as f32,
                ),
                ..Default::default()
            },
            VisibilityBundle {
                visibility,
                ..Default::default()
            },
        ));

        Ok(ChunkEntity(entity.id()))
    }

    /// Extracts the heightmaps from the chunk's NBT data.
    fn unwrap_heightmaps(nbt: Nbt) -> Option<(Vec<i64>, Vec<i64>)> {
        let root = nbt.as_compound()?.get("")?.as_compound()?;

        let motion = root.get("MOTION_BLOCKING")?.as_long_array()?;
        let world = root.get("WORLD_SURFACE")?.as_long_array()?;

        Some((motion.to_owned(), world.to_owned()))
    }

    /// Insert data into a chunk section.
    #[allow(dead_code)]
    pub fn update_section<V: GlobalPalette>(
        &mut self,
        position: ChunkSectionPos,
        data: SectionDataPacket,
    ) {
        self.sections.write()[position.y as usize].insert_data::<V>(data);
    }

    /// Get the total number of blocks in the chunk.
    pub fn block_count(&self) -> u32 {
        self.sections
            .read()
            .iter()
            .fold(0u32, |acc, section| acc + section.block_count as u32)
    }
}

impl From<Entity> for ChunkEntity {
    fn from(entity: Entity) -> Self { Self(entity) }
}

impl From<ChunkEntity> for Entity {
    fn from(entity: ChunkEntity) -> Self { entity.0 }
}
