use mc_rs_proto::buffer::{Decode, DecodeError, VarDecode};

use super::{global_palette::GlobalPalette, section::DataKind, CHUNK_SIZE, SECTION_HEIGHT};

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

    /// Get the global id of the palette type at the given index
    fn get_type_index(&self, index: u32) -> u32 {
        match &self.kind {
            PaletteKind::Single(id) => *id,
            PaletteKind::Array(ids) | PaletteKind::Bimap(ids) => ids[index as usize],
            PaletteKind::Global => index,
        }
    }

    /// Get the contents of the palette, converted to global ids
    pub fn get_data(&self) -> Vec<u32> {
        if let PaletteKind::Single(id) = &self.kind {
            return vec![*id; CHUNK_SIZE * CHUNK_SIZE * SECTION_HEIGHT];
        }

        let mut block_ids = Vec::with_capacity(self.data.len() * (64 / self.bits) as usize);
        for long in self.data.iter() {
            for i in 0..(64 / self.bits) {
                let index = (long >> (i * self.bits)) & ((1 << self.bits) - 1);
                block_ids.push(self.get_type_index(index as u32));
            }
        }

        block_ids
    }

    /// Insert data into the palette at the given position
    pub(super) fn insert_data<V: GlobalPalette>(
        &mut self,
        _x: u8,
        _y: u8,
        _z: u8,
        data: u32,
    ) -> i8 {
        let state_id = V::to_global_block(data);

        match &mut self.kind {
            PaletteKind::Single(id) => {
                if state_id != *id {
                    self.kind = PaletteKind::Array(vec![*id, state_id]);

                    // TODO: Update palette data
                }
            }
            PaletteKind::Array(ids) | PaletteKind::Bimap(ids) => {
                let _index = if let Some(index) = ids.iter().find(|&&id| id == state_id).copied() {
                    index
                } else {
                    ids.push(state_id);
                    ids.len() as u32 - 1
                };

                // TODO: Update palette data
            }
            PaletteKind::Global => {
                // TODO: Update palette data
            }
        }

        // TODO: Return block delta
        0
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
