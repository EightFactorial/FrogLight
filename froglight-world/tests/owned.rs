//! TODO
#![no_std]

#[cfg(feature = "alloc")]
use bitvec::vec::BitVec;
#[cfg(feature = "alloc")]
use froglight_world::{
    chunk::{
        Section,
        section::{SectionData, SectionPalette},
    },
    component::{ChunkBlockPos, SectionBlockPos},
    prelude::{BlockPos, NaiveChunk},
};

#[test]
#[cfg(feature = "alloc")]
fn chunk() {
    // An empty chunk with no blocks.
    let chunk = NaiveChunk::new_empty_large();
    let offset = chunk.height_offset();

    for y in chunk.height_range() {
        for z in 0..16 {
            for x in 0..16 {
                let position = BlockPos::new_xyz(x, y, z);
                let chunk_position = ChunkBlockPos::try_from_blockpos(position, offset).unwrap();
                assert_eq!(chunk.get_raw_block(position), Some(0));
                assert_eq!(chunk.get_raw_block_pos(chunk_position), Some(0));
                assert_eq!(chunk.get_raw_biome(position), Some(0));
                assert_eq!(chunk.get_raw_biome_pos(chunk_position), Some(0));
            }
        }
    }

    for id in chunk.iter_raw_blocks() {
        assert_eq!(id, 0);
    }
    for id in chunk.iter_raw_biomes() {
        assert_eq!(id, 0);
    }
}

#[test]
#[cfg(feature = "alloc")]
fn empty() {
    // An empty section with no blocks.
    let section = unsafe {
        Section::new_unchecked(
            0,
            SectionData::new_unchecked(0, SectionPalette::Single(0), BitVec::EMPTY),
            SectionData::new_unchecked(0, SectionPalette::Single(0), BitVec::EMPTY),
        )
    };

    assert_eq!(section.block_count(), 0);
    assert_eq!(section.block_data().bits_per_entry(), 0);
    assert_eq!(section.biome_data().bits_per_entry(), 0);

    for index in 0..(16 * 16 * 16) {
        let pos = SectionBlockPos::new_index(index);
        assert_eq!(section.get_raw_block(pos), 0);
        assert_eq!(section.block_data().get_index(usize::from(index)), Some(0));
    }
    for index in 0..(16 * 16 * 16) / (4 * 4 * 4) {
        let pos = SectionBlockPos::new_index(index);
        assert_eq!(section.get_raw_biome(pos), 0);
        assert_eq!(section.biome_data().get_index(usize::from(index)), Some(0));
    }

    for id in section.iter_raw_blocks() {
        assert_eq!(id, 0);
    }
    for id in section.iter_raw_biomes() {
        assert_eq!(id, 0);
    }
}
