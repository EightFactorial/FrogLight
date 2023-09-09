use std::{io::Cursor, sync::Arc};

use azalea_nbt::Nbt;
use bevy::prelude::*;
use mc_rs_proto::{
    buffer::DecodeError,
    types::{
        packets::chunk_data::{ChunkDataPacket, SectionDataPacket},
        position::{ChunkPos, ChunkSectionPos},
    },
};
use parking_lot::RwLock;

use crate::systems::{
    blocks::BlockData,
    world::{
        global_palette::GlobalPalette, task::ChunkTask, WorldType, Worlds, CHUNK_SIZE,
        CHUNK_VERT_DISPLACEMENT, SECTION_COUNT,
    },
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

    /// Regenerate chunk meshes.
    pub fn update_chunk(
        query: Query<Ref<Chunk>>,
        block_data: Res<BlockData>,
        mut worlds: ResMut<Worlds>,
        mut commands: Commands,
    ) {
        for chunk in query.iter() {
            if chunk.is_changed() {
                let Some(world) = worlds.get_world_mut(&chunk.world_type) else {
                    error!(
                        "Failed to get world {:?} with chunk {:?}!",
                        chunk.world_type, chunk.position
                    );
                    continue;
                };

                // If the chunk is empty, despawn it
                if chunk.block_count() == 0 {
                    if let Some(entity) = world.get_chunk_id(&chunk.position) {
                        let entity = **entity;
                        commands.entity(entity).despawn_recursive();
                        world.remove(&chunk.position);
                    } else {
                        error!(
                            "Failed to get chunk entity for empty chunk {:?}",
                            chunk.position
                        );
                    }
                    continue;
                }

                let mut neighbors = [None, None, None, None];
                for (pos, val) in chunk
                    .position
                    .around()
                    .into_iter()
                    .zip(neighbors.iter_mut())
                {
                    if let Some(chunk) = world.get_chunk_ref(&query, &pos) {
                        if chunk.block_count() != 0 {
                            *val = Some(chunk.sections.clone());
                        }
                    }
                }

                if let Some(entity) = world.get_chunk_id(&chunk.position) {
                    commands.entity(**entity).insert(ChunkTask::create(
                        chunk.sections.clone(),
                        neighbors,
                        block_data.clone(),
                    ));
                } else {
                    error!(
                        "Failed to get chunk entity for changed chunk {:?}",
                        chunk.position
                    );
                }
            }

            if chunk.is_added() {
                // For each neighboring chunk
                let world = worlds.get_world(&chunk.world_type).unwrap();
                for neighbor_pos in chunk.position.around().into_iter() {
                    if let Some(neighbor_chunk) = world.get_chunk_ref(&query, &neighbor_pos) {
                        // Skip chunks with no blocks
                        if neighbor_chunk.block_count() == 0 {
                            continue;
                        }

                        // Get that chunk's neighbors
                        let mut neighbors = [None, None, None, None];
                        for (pos, val) in
                            neighbor_pos.around().into_iter().zip(neighbors.iter_mut())
                        {
                            if let Some(chunk) = world.get_chunk_ref(&query, &pos) {
                                *val = Some(chunk.sections.clone());
                            }
                        }

                        // Update the neighbor chunk mesh
                        if let Some(entity) = world.get_chunk_id(&neighbor_pos) {
                            commands.entity(**entity).insert(ChunkTask::create(
                                neighbor_chunk.sections.clone(),
                                neighbors,
                                block_data.clone(),
                            ));
                        } else {
                            error!("Failed to get chunk entity for added chunk {neighbor_pos:?}");
                        }
                    }
                }
            }
        }
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
