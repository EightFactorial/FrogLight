use bevy::prelude::*;
use mc_rs_proto::buffer::{Decode, DecodeError};

use super::{global_palette::GlobalPalette, palette::Palette};

/// A marker component for sections
#[derive(Debug, Clone, Copy, Component)]
pub struct SectionComponent;

impl SectionComponent {
    /// Despawn all section entities that are not attached to a chunk
    pub(super) fn despawn_orphans(
        query: Query<Entity, (With<SectionComponent>, Without<Parent>)>,
        mut commands: Commands,
    ) {
        for entity in query.iter() {
            warn!("Despawning orphaned section entity {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Section {
    pub block_count: u16,
    pub block_palette: Palette,
    pub biome_palette: Palette,
}

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

    pub fn get_blocks(&self) -> Vec<u32> { self.block_palette.get_values() }

    pub fn get_biomes(&self) -> Vec<u32> { self.biome_palette.get_values() }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataKind {
    Block,
    Biome,
}
