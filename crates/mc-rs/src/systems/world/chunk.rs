use std::io::Cursor;

use azalea_nbt::Nbt;
use bevy::prelude::{Commands, Component, Deref, DerefMut, Entity};
use mc_rs_proto::{buffer::DecodeError, types::packets::chunk_data::ChunkDataPacket};

use super::{
    global_palette::GlobalPalette,
    section::{ChunkSection, Section},
    task::SectionTask,
    SECTION_COUNT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct ChunkEntity(pub Entity);

#[derive(Debug, Default, Clone, Component)]
pub struct Chunk {
    pub sections: [Section; SECTION_COUNT],
    pub section_ids: [Option<Entity>; SECTION_COUNT],
    pub motion_blocking: Vec<i64>,
    pub world_surface: Vec<i64>,
}

impl Chunk {
    pub(super) fn decode<V: GlobalPalette>(
        chunk_data: ChunkDataPacket,
        commands: &mut Commands,
    ) -> Result<Self, DecodeError> {
        let mut cursor = Cursor::new(chunk_data.data.as_slice());

        let mut sections = Vec::with_capacity(SECTION_COUNT);
        let mut section_ids = Vec::with_capacity(SECTION_COUNT);

        for (i, section_id) in section_ids.iter_mut().enumerate().take(SECTION_COUNT) {
            let section = Section::decode::<V>(&mut cursor)?;

            if section.block_count != 0 {
                let id = commands
                    .spawn((ChunkSection(i as isize), SectionTask::new(section.clone())))
                    .id();
                *section_id = Some(id);
            }

            sections.push(section);
        }

        let (motion_blocking, world_surface) = Self::unwrap_heightmaps(chunk_data.heightmaps)
            .unwrap_or_else(|| {
                log::warn!("Failed to unwrap chunk heightmaps");
                Default::default()
            });

        Ok(Self {
            sections: sections.try_into().unwrap(),
            section_ids: section_ids.try_into().unwrap(),
            motion_blocking,
            world_surface,
        })
    }

    fn unwrap_heightmaps(nbt: Nbt) -> Option<(Vec<i64>, Vec<i64>)> {
        let root = nbt.as_compound()?.get("")?.as_compound()?;

        let motion = root.get("MOTION_BLOCKING")?.as_long_array()?;
        let world = root.get("WORLD_SURFACE")?.as_long_array()?;

        Some((motion.to_owned(), world.to_owned()))
    }
}
