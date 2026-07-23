//! TODO

use froglight_world::{
    component::{ChunkBlockPos, SectionBlockPos},
    prelude::{BlockPos, NaiveChunk},
    section::Section,
};

#[test]
fn chunk() {
    // An empty chunk with no blocks.
    let mut chunk = NaiveChunk::new_empty_large();
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

    // Set `[8, 8, 8]` to `1`.
    let position = BlockPos::new_xyz(8, 8, 8);
    let existing = chunk.set_raw_block(position, 1, is_air, is_fluid).unwrap();

    assert_eq!(existing, 0);
    assert_eq!(chunk.get_raw_block(position), Some(1));

    // Check that adjacent blocks are still `0`.
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(7, 8, 8)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(8, 7, 8)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(8, 8, 7)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(7, 7, 8)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(7, 8, 7)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(8, 7, 7)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(7, 7, 7)), Some(0));

    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(9, 8, 8)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(8, 9, 8)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(8, 8, 9)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(9, 9, 8)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(9, 8, 9)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(8, 9, 9)), Some(0));
    assert_eq!(chunk.get_raw_block(BlockPos::new_xyz(9, 9, 9)), Some(0));
}

#[test]
fn section() {
    // An empty section with no blocks.
    let mut section = Section::empty();

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

    // Set `[8, 8, 8]` to `1`.
    let position = SectionBlockPos::new_xyz(8, 8, 8);
    let existing = section.set_raw_block(position, 1, is_air, is_fluid);

    assert_eq!(existing, 0);
    assert_eq!(section.get_raw_block(position), 1);

    // Check that adjacent blocks are still `0`.
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(7, 8, 8)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(8, 7, 8)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(8, 8, 7)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(7, 7, 8)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(7, 8, 7)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(8, 7, 7)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(7, 7, 7)), 0);

    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(9, 8, 8)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(8, 9, 8)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(8, 8, 9)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(9, 9, 8)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(9, 8, 9)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(8, 9, 9)), 0);
    assert_eq!(section.get_raw_block(SectionBlockPos::new_xyz(9, 9, 9)), 0);
}

// ------------------------------------------------------------------------------------------------

fn is_air(id: u32) -> bool { id == 0 }

fn is_fluid(_: u32) -> bool { false }
