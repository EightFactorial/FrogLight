use std::io::Cursor;

use azalea_nbt::Nbt;
use bevy::prelude::{Component, Deref, DerefMut, Entity, Visibility, VisibilityBundle};
use mc_rs_proto::{buffer::DecodeError, types::packets::chunk_data::ChunkDataPacket};

use crate::systems::blocks::block_list::Blocks;

use super::{
    global_palette::GlobalPalette,
    section::Section,
    task::{SectionMarker, SectionTask},
    SECTION_COUNT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct ChunkEntity(pub Entity);

#[derive(Debug, Default, Clone, Component)]
pub struct Chunk {
    pub sections: [Section; SECTION_COUNT],
    pub motion_blocking: Vec<i64>,
    pub world_surface: Vec<i64>,
}

impl Chunk {
    /// Decodes a chunk from a chunk data packet.
    pub(super) fn decode<V: GlobalPalette>(
        chunk_data: ChunkDataPacket,
        blocks: &Blocks,
        world: &mut bevy::ecs::world::World,
    ) -> Result<(Self, Vec<Entity>), DecodeError> {
        let mut cursor = Cursor::new(chunk_data.data.as_slice());

        let mut sections = Vec::with_capacity(SECTION_COUNT);
        let mut section_entities = Vec::with_capacity(SECTION_COUNT);

        for index in 0..SECTION_COUNT {
            let section = Section::decode::<V>(&mut cursor)?;

            if section.block_count != 0 {
                let section = world.spawn((
                    VisibilityBundle {
                        visibility: Visibility::Visible,
                        ..Default::default()
                    },
                    SectionMarker(index),
                    SectionTask::new(section.clone(), blocks.clone()),
                ));

                section_entities.push(section.id());
            }

            sections.push(section);
        }

        let (motion_blocking, world_surface) = Self::unwrap_heightmaps(chunk_data.heightmaps)
            .unwrap_or_else(|| {
                log::warn!("Failed to unwrap chunk heightmaps");
                Default::default()
            });

        Ok((
            Self {
                sections: sections.try_into().unwrap(),
                motion_blocking,
                world_surface,
            },
            section_entities,
        ))
    }

    /// Extracts the heightmaps from the chunk's NBT data.
    fn unwrap_heightmaps(nbt: Nbt) -> Option<(Vec<i64>, Vec<i64>)> {
        let root = nbt.as_compound()?.get("")?.as_compound()?;

        let motion = root.get("MOTION_BLOCKING")?.as_long_array()?;
        let world = root.get("WORLD_SURFACE")?.as_long_array()?;

        Some((motion.to_owned(), world.to_owned()))
    }
}
