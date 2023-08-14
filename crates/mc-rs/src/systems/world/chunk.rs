use std::{
    io::Cursor,
    sync::{Arc, RwLock},
};

use azalea_nbt::Nbt;
use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBody;
use mc_rs_proto::{
    buffer::DecodeError,
    types::{
        packets::chunk_data::{ChunkDataPacket, SectionDataPacket},
        position::{ChunkPos, ChunkSectionPos},
    },
};

use crate::systems::blocks::block_list::Blocks;

use super::{
    global_palette::GlobalPalette, section::Section, task::ChunkTask, WorldType, Worlds,
    CHUNK_SIZE, CHUNK_VERT_DISPLACEMENT, SECTION_COUNT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct ChunkEntity(pub Entity);

#[derive(Debug, Default, Clone, Component)]
pub struct Chunk {
    pub sections: Arc<RwLock<[Section; SECTION_COUNT]>>,
    pub motion_blocking: Vec<i64>,
    pub world_surface: Vec<i64>,
    pub world_type: WorldType,
    pub position: ChunkPos,
}

impl Chunk {
    /// Decodes a chunk from a chunk data packet.
    pub(super) fn decode<V: GlobalPalette>(
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
            RigidBody::Fixed,
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
                    -CHUNK_VERT_DISPLACEMENT as f32,
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
    pub fn update_section<V: GlobalPalette>(
        &mut self,
        position: ChunkSectionPos,
        data: SectionDataPacket,
    ) {
        self.sections.write().unwrap()[position.y as usize].insert_data::<V>(data);
    }

    /// Regenerate the mesh for chunks that have been updated.
    pub(super) fn update_chunk(
        query: Query<(Entity, &Chunk), Changed<Chunk>>,
        worlds: Res<Worlds>,
        blocks: Res<Blocks>,
        mut commands: Commands,
    ) {
        for (entity, chunk) in query.iter() {
            let mut neighbors = [None, None, None, None];
            let world = worlds.get_world(&chunk.world_type).unwrap();

            for (pos, val) in chunk
                .position
                .around()
                .into_iter()
                .zip(neighbors.iter_mut())
            {
                if let Some(entity) = world.get_chunk(pos) {
                    if let Ok((_, chunk)) = query.get(**entity) {
                        *val = Some(chunk.clone());
                    }
                }
            }

            commands.entity(entity).insert(ChunkTask::create(
                chunk.clone(),
                neighbors,
                blocks.clone(),
            ));
        }
    }

    /// Regenerate the mesh for chunks that have neighbors added.
    pub(super) fn added_chunk(
        query: Query<(Entity, &Chunk, Ref<Chunk>)>,
        worlds: Res<Worlds>,
        blocks: Res<Blocks>,
        mut commands: Commands,
    ) {
        for (_, chunk, chunk_ref) in query.iter() {
            // For newly added chunks
            if !chunk_ref.is_added() {
                continue;
            }

            // For each neighboring chunk
            let world = worlds.get_world(&chunk.world_type).unwrap();
            for neighbor_pos in chunk.position.around().into_iter() {
                if let Some(entity) = world.get_chunk(neighbor_pos) {
                    if let Ok((_, neighbor_chunk, _)) = query.get(**entity) {
                        // Get that chunk's neighbors
                        let mut neighbors = [None, None, None, None];
                        for (pos, val) in
                            neighbor_pos.around().into_iter().zip(neighbors.iter_mut())
                        {
                            if pos == chunk.position {
                                continue;
                            }

                            if let Some(entity) = world.get_chunk(pos) {
                                if let Ok((_, chunk, _)) = query.get(**entity) {
                                    *val = Some(chunk.clone());
                                }
                            }
                        }

                        // Update the neighbor chunk mesh
                        commands.entity(**entity).insert(ChunkTask::create(
                            neighbor_chunk.clone(),
                            neighbors,
                            blocks.clone(),
                        ));
                    }
                }
            }
        }
    }
}
