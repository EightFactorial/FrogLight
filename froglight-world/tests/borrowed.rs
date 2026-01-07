//! TODO
#![no_std]

use bitvec::slice::BitSlice;
use froglight_world::{
    borrowed::{
        BorrowedChunk, BorrowedSection,
        section::{BorrowedPalette, BorrowedSectionData},
    },
    component::{ChunkBlockPos, SectionBlockPos},
    prelude::BlockPos,
};

#[test]
fn chunk() {
    // An empty chunk with no blocks.
    let chunk = BorrowedChunk::new_empty_large();
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
fn empty() {
    // An empty section with no blocks.
    let section = unsafe {
        BorrowedSection::new_unchecked(
            0,
            BorrowedSectionData::new_unchecked(0, BorrowedPalette::Single(0), BitSlice::empty()),
            BorrowedSectionData::new_unchecked(0, BorrowedPalette::Single(0), BitSlice::empty()),
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
