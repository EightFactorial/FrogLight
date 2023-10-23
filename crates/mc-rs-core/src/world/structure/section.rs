use bevy::prelude::*;
use mc_rs_protocol::{
    buffer::{Decode, DecodeError},
    types::packets::chunk_data::SectionDataPacket,
};

use crate::world::palette::GlobalPalette;

use super::palette::Palette;

/// A marker component for sections
#[derive(Debug, Clone, Copy, Component)]
pub struct SectionComponent;

impl SectionComponent {
    /// Despawn all section entities that are not attached to a chunk
    pub fn despawn_orphans(
        query: Query<Entity, (With<SectionComponent>, Without<Parent>)>,
        mut commands: Commands,
    ) {
        for entity in query.iter() {
            warn!("Despawning orphaned section entity {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub block_count: u16,
    pub block_palette: Palette,
    pub biome_palette: Palette,
}

#[allow(dead_code)]
impl Section {
    pub(super) fn decode<V: GlobalPalette>(
        buf: &mut impl std::io::Read,
    ) -> Result<Self, DecodeError> {
        Ok(Self {
            block_count: u16::decode(buf)?,
            block_palette: Palette::decode::<V>(DataKind::Block, buf)?,
            biome_palette: Palette::decode::<V>(DataKind::Biome, buf)?,
        })
    }

    /// Get the blocks in this section
    pub fn get_blocks(&self) -> Vec<u32> { self.block_palette.get_data() }

    /// Get the biomes in this section
    pub fn get_biomes(&self) -> Vec<u32> { self.biome_palette.get_data() }

    /// Update the section with the given data
    pub(super) fn insert_data<V: GlobalPalette>(&mut self, data: SectionDataPacket) {
        let delta = self
            .block_palette
            .insert_data::<V>(data.x, data.y, data.z, data.state);

        match (delta > 0, delta < 0) {
            (true, false) => self.block_count += delta as u16,
            (false, true) => self.block_count -= u16::try_from(delta.abs()).unwrap(),
            _ => (),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataKind {
    Block,
    Biome,
}
