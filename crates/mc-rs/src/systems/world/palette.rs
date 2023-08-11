use mc_rs_proto::buffer::{Decode, DecodeError, VarDecode};

use super::{global_palette::GlobalPalette, section::DataKind};

#[derive(Debug, Default, Clone)]
pub struct Palette {
    pub kind: PaletteKind,
    pub data: Vec<u64>,
    pub bits: u8,
}

impl Palette {
    pub(super) fn decode<V: GlobalPalette>(
        kind: DataKind,
        buf: &mut impl std::io::Read,
    ) -> Result<Self, DecodeError> {
        let bits = u8::decode(buf)?;

        let pal = Self {
            kind: PaletteKind::decode(&kind, bits, buf)?,
            data: Vec::decode(buf)?,
            bits,
        };

        match kind {
            DataKind::Block => Ok(pal.block_ids::<V>()),
            DataKind::Biome => Ok(pal.biome_ids::<V>()),
        }
    }

    /// Convert the block ids in the palette to global ids
    fn block_ids<V: GlobalPalette>(mut self) -> Self {
        match self.kind {
            PaletteKind::Single(id) => self.kind = PaletteKind::Single(V::to_global_block(id)),
            PaletteKind::Array(ids) => {
                self.kind = PaletteKind::Array(V::batch_to_global_block(ids))
            }
            PaletteKind::Bimap(ids) => {
                self.kind = PaletteKind::Bimap(V::batch_to_global_block(ids))
            }
            PaletteKind::Global => {
                todo!()
            }
        }

        self
    }

    /// Convert the biome ids in the palette to global ids
    fn biome_ids<V: GlobalPalette>(mut self) -> Self {
        match self.kind {
            PaletteKind::Single(id) => self.kind = PaletteKind::Single(V::to_global_biome(id)),
            PaletteKind::Array(ids) => {
                self.kind = PaletteKind::Array(V::batch_to_global_biome(ids))
            }
            PaletteKind::Bimap(ids) => {
                self.kind = PaletteKind::Bimap(V::batch_to_global_biome(ids))
            }
            PaletteKind::Global => {
                todo!()
            }
        }

        self
    }
}

#[derive(Debug, Clone)]
pub enum PaletteKind {
    Single(u32),
    Array(Vec<u32>),
    Bimap(Vec<u32>),
    Global,
}

impl PaletteKind {
    fn decode(
        kind: &DataKind,
        bits: u8,
        buf: &mut impl std::io::Read,
    ) -> Result<Self, DecodeError> {
        match (kind, bits) {
            (_, 0) => Ok(Self::Single(u32::var_decode(buf)?)),
            (DataKind::Biome, 1..=3) | (DataKind::Block, 1..=4) => {
                Ok(Self::Array(Vec::var_decode(buf)?))
            }
            (DataKind::Block, 5..=8) => Ok(Self::Bimap(Vec::var_decode(buf)?)),
            _ => Ok(Self::Global),
        }
    }
}

impl Default for PaletteKind {
    fn default() -> Self { PaletteKind::Single(0u32) }
}
