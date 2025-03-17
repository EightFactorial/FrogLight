use super::{ArrayChunkStorage, ChunkStorage};
use crate::{position::SectionBlockPos, prelude::*};

/// Test the default empty chunk.
#[test]
fn chunk_default() {
    let chunk = Chunk::new(ChunkStorage::Large(ArrayChunkStorage::default()));
    let (min, max) = (chunk.storage.position_min(), chunk.storage.position_max());

    // Check that all blocks are air.
    assert!(chunk.storage.sections_ref().iter().all(|s| s.blocks() == 0));
    for y in min.y()..=max.y() {
        for z in min.z()..=max.z() {
            for x in min.x()..=max.x() {
                assert_eq!(chunk.get_raw_block(BlockPos::new(x, y, z)), Some(0));
            }
        }
    }
}

/// Test resolving the default empty chunk.
///
/// # Note
/// This is extremely inefficient!
/// You should resolve the palette first, then use that to get the real blocks.
#[test]
#[cfg(all(feature = "block", feature = "v1_21_4"))]
fn chunk_default_blocks() {
    use froglight_block::{generated::v1_21_4::VersionBlocks, prelude::*};
    use froglight_common::{vanilla::Vanilla, version::V1_21_4};

    let storage = BlockStorage::new();
    let chunk = Chunk::new(ChunkStorage::Large(ArrayChunkStorage::default()));
    let (min, max) = (chunk.storage.position_min(), chunk.storage.position_max());

    assert!(chunk.storage.sections_ref().iter().all(|s| s.blocks() == 0));
    for y in min.y()..=max.y() {
        for z in min.z()..=max.z() {
            for x in min.x()..=max.x() {
                let pos = BlockPos::new(x, y, z);

                let block = chunk.get_block_trait(pos, &storage).unwrap();
                assert_eq!(block.identifier(), "minecraft:air");

                let block = chunk.get_block_untyped(pos, &storage).unwrap();
                assert_eq!(block.identifier(), "minecraft:air");

                let block = chunk.get_block_typed::<V1_21_4, Vanilla>(pos, &storage).unwrap();
                assert!(matches!(block, VersionBlocks::Air(..)));
            }
        }
    }
}

/// Test resolving blocks in a chunk.
///
/// # Note
/// This is extremely inefficient!
/// You should resolve the palette first, then use that to get the real blocks.
#[test]
#[cfg(all(feature = "block", feature = "v1_21_4"))]
fn chunk_blocks() {
    use froglight_block::prelude::*;
    use froglight_common::version::V1_21_4;

    let storage = BlockStorage::new();
    let mut chunk = Chunk::new(ChunkStorage::Large(ArrayChunkStorage::default()));
    let (min, max) = (chunk.storage.position_min(), chunk.storage.position_max());

    // Fill the chunk with assorted blocks.
    for y in min.y()..=max.y() {
        for z in min.z()..=max.z() {
            for x in min.x()..=max.x() {
                let pos = BlockPos::new(x, y, z);
                let pos_id = SectionBlockPos::from_block(pos).into_index() as u32;
                println!("Setting block at {pos} to {pos_id}...");

                let block = storage.get_untyped(GlobalBlockId::new_unchecked(pos_id)).unwrap();
                assert_eq!(
                    chunk.set_block(pos, block, &storage),
                    Some(0),
                    "Overwriting block at {pos}?"
                );
                assert_eq!(
                    chunk.get_raw_block(pos),
                    Some(pos_id),
                    "Failed to write block at {pos}!"
                );
            }
        }
    }

    // Assert that all blocks match again.
    for y in min.y()..=max.y() {
        for z in min.z()..=max.z() {
            for x in min.x()..=max.x() {
                let pos = BlockPos::new(x, y, z);
                let pos_id = SectionBlockPos::from_block(pos).into_index() as u32;

                let block = chunk.get_block_untyped::<V1_21_4>(pos, &storage).unwrap();
                let block_id: u32 = storage.get_global(block).unwrap().into();
                assert_eq!(
                    pos_id, block_id,
                    "Block mismatch at {pos}, expected {pos_id}, found {block_id}!",
                );
            }
        }
    }
}

/// Test a `SectionPalette::Single`.
#[test]
fn section_single() {
    let section = Section::default();
    assert_eq!(section.blocks(), 0);

    // Check all blocks
    for index in 0..4096 {
        assert_eq!(section.get_block(SectionBlockPos::from_index(index)), 0);
    }
}

/// Test growing a `SectionPalette::Single`
/// into a `SectionPalette::Vector`.
#[test]
fn section_vector() {
    let mut section = Section::default();
    assert_eq!(section.blocks(), 0);

    // Set all blocks and check them
    for index in 0..256 {
        let pos = SectionBlockPos::from_index(index);
        assert_eq!(section.set_block(pos, index as u32), 0, "Weird return value at {index}?");
        assert_eq!(section.get_block(pos), index as u32, "Invalid block at {index}!");
    }
    // Check all blocks one more time
    for index in 0..256 {
        let pos = SectionBlockPos::from_index(index);
        assert_eq!(section.get_block(pos), index as u32, "Invalid block at {index}!");
    }
}

/// Test growing a `SectionPalette::Single`
/// all the way into a `SectionPalette::Global`.
#[test]
fn section_global() {
    let mut section = Section::default();
    assert_eq!(section.blocks(), 0);

    // Set all blocks and check them
    for index in 0..4096 {
        let pos = SectionBlockPos::from_index(index);
        assert_eq!(section.set_block(pos, index as u32), 0, "Weird return value at {index}?");
        assert_eq!(section.get_block(pos), index as u32, "Invalid block at {index}!");
    }
    // Check all blocks one more time
    for index in 0..4096 {
        let pos = SectionBlockPos::from_index(index);
        assert_eq!(section.get_block(pos), index as u32, "Invalid block at {index}!");
    }
}
