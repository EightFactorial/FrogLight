use bevy::prelude::{Component, Deref, DerefMut};
use mc_rs_proto::buffer::{Decode, DecodeError};

use super::{global_palette::GlobalPalette, palette::Palette};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut, Component,
)]
pub struct ChunkSection(pub isize);

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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataKind {
    Block,
    Biome,
}
