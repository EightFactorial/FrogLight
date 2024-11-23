use bitvec::{order::Msb0, vec::BitVec};
use froglight_protocol::common::SectionBlockPosition;

use crate::{BlockContainer, ChunkSection, ContainerPalette};

#[test]
fn bitvec_size() {
    for (i, size) in [
        4096, 8192, 12544, 16384, 21888, 26240, 29184, 32768, 37504, 43712, 52480, 52480, 65536,
        65536, 65536, 65536, 87424, 87424, 87424, 87424, 87424, 131_072, 131_072, 131_072, 131_072,
        131_072, 131_072, 131_072, 131_072, 131_072, 131_072, 131_072,
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(BlockContainer::data_size_bits(i + 1), size);
    }
}

#[test]
fn entry_bitsize() {
    for (i, size) in [
        1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 6,
    ]
    .into_iter()
    .enumerate()
    {
        // If there were `X` entries, the number of bits required would be `Y`.
        assert_eq!(BlockContainer::vector_bits_required(i + 1), size);
        // If the maximum value was `X`, the number of bits required would be `Y`.
        assert_eq!(BlockContainer::global_bits_required(u32::try_from(i + 1).unwrap()), size);
    }
}

#[test]
fn empty_container() {
    let container = BlockContainer::default();

    // Check that the container is empty.
    assert_eq!(container.bits, 0);
    assert_eq!(container.palette, ContainerPalette::Single(0));
    assert_eq!(container.data, BitVec::<u64, Msb0>::EMPTY);

    // Check that it's possible to get the first and last values.
    assert_eq!(container.get_data(&SectionBlockPosition::MIN), 0);
    assert_eq!(container.get_data(&SectionBlockPosition::MAX), 0);

    // Check that all values are 0.
    for index in 0..ChunkSection::VOLUME {
        let pos = SectionBlockPosition::from_index(index as usize);
        assert_eq!(container.get_data(&pos), 0);
    }
}

#[test]
fn single_container() {
    let mut container = BlockContainer::default();
    assert_eq!(container.bits, 0);

    // Check that it's possible to get the first and last values.
    assert_eq!(container.get_data(&SectionBlockPosition::MIN), 0);
    assert_eq!(container.get_data(&SectionBlockPosition::MAX), 0);

    // Set the value at the given position.
    let set_pos = SectionBlockPosition::new(2, 4, 8);
    let value = 5;

    // Set the value and check that it's set.
    assert_eq!(container.set_data(&set_pos, value), 0);
    assert_eq!(container.get_data(&set_pos), value);
    assert!(matches!(container.palette, ContainerPalette::Vector(_)));
    assert_eq!(container.bits, 1);

    // Check that all other values are 0.
    for index in 0..ChunkSection::VOLUME {
        let pos = SectionBlockPosition::from_index(index as usize);
        if pos != set_pos {
            assert_eq!(container.get_data(&pos), 0);
        }
    }

    // Set the value again and make sure nothing changed.
    assert_eq!(container.set_data(&set_pos, value), value);
    assert_eq!(container.get_data(&set_pos), value);
    assert!(matches!(container.palette, ContainerPalette::Vector(_)));
    assert_eq!(container.bits, 1);

    // Check that all other values are 0.
    for index in 0..ChunkSection::VOLUME {
        let pos = SectionBlockPosition::from_index(index as usize);
        if pos != set_pos {
            assert_eq!(container.get_data(&pos), 0);
        }
    }
}

#[test]
fn vector_container() {
    let mut container = BlockContainer::default();
    assert_eq!(container.bits, 0);

    // Create a position and value to set.
    let first_pos = SectionBlockPosition::new(0, 0, 0);
    let first_val = 5;

    // Set the value and check that it's set.
    assert_eq!(container.set_data(&first_pos, first_val), 0);
    assert_eq!(container.get_data(&first_pos), first_val);
    assert!(matches!(container.palette, ContainerPalette::Vector(_)));
    assert_eq!(container.bits, 1);

    // Create a second position and value to set.
    let second_pos = SectionBlockPosition::new(1, 0, 0);
    let second_val = 6;

    // Get the first value and check that it's still set.
    assert_eq!(container.get_data(&first_pos), first_val);
    assert_eq!(container.bits, 1);
    // Set the second value and check that it's set.
    assert_eq!(container.set_data(&second_pos, second_val), 0);
    assert_eq!(container.get_data(&second_pos), second_val);
    assert!(matches!(container.palette, ContainerPalette::Vector(_)));
    assert_eq!(container.bits, 2);

    // Check that all other values are 0.
    for index in 0..ChunkSection::VOLUME {
        let pos = SectionBlockPosition::from_index(index as usize);
        if pos != first_pos && pos != second_pos {
            assert_eq!(container.get_data(&pos), 0);
        }
    }

    // Create a third position and value to set.
    let third_pos = SectionBlockPosition::new(2, 0, 0);
    let third_val = 7;

    // Get the first and second values and check that they're still set.
    assert_eq!(container.get_data(&first_pos), first_val);
    assert_eq!(container.get_data(&second_pos), second_val);
    // Set the third value and check that it's set.
    assert_eq!(container.set_data(&third_pos, third_val), 0);
    assert_eq!(container.get_data(&third_pos), third_val);
    assert!(matches!(container.palette, ContainerPalette::Vector(_)));
    assert_eq!(container.bits, 2);

    // Check that all other values are 0.
    for index in 0..ChunkSection::VOLUME {
        let pos = SectionBlockPosition::from_index(index as usize);
        if pos != first_pos && pos != second_pos && pos != third_pos {
            assert_eq!(container.get_data(&pos), 0);
        }
    }

    // Create a fourth position and value to set.
    let fourth_pos = SectionBlockPosition::new(2, 8, 6);
    let fourth_val = 513;

    // Get the first, second, and third values and check that they're still set.
    assert_eq!(container.get_data(&first_pos), first_val);
    assert_eq!(container.get_data(&second_pos), second_val);
    assert_eq!(container.get_data(&third_pos), third_val);
    // Set the fourth value and check that it's set.
    assert_eq!(container.set_data(&fourth_pos, fourth_val), 0);
    assert_eq!(container.get_data(&fourth_pos), fourth_val);
    assert!(matches!(container.palette, ContainerPalette::Vector(_)));
    assert_eq!(container.bits, 3);

    // Check that all other values are 0.
    for index in 0..ChunkSection::VOLUME {
        let pos = SectionBlockPosition::from_index(index as usize);
        if pos != first_pos && pos != second_pos && pos != third_pos && pos != fourth_pos {
            assert_eq!(container.get_data(&pos), 0);
        }
    }

    // Create a fifth position and value to set.
    let fifth_pos = SectionBlockPosition::new(3, 15, 15);
    let fifth_val = 8192;

    // Get the first, second, third, and fourth values and check that they're still
    // set.
    assert_eq!(container.get_data(&first_pos), first_val);
    assert_eq!(container.get_data(&second_pos), second_val);
    assert_eq!(container.get_data(&third_pos), third_val);
    assert_eq!(container.get_data(&fourth_pos), fourth_val);
    // Set the fifth value and check that it's set.
    assert_eq!(container.set_data(&fifth_pos, fifth_val), 0);
    assert_eq!(container.get_data(&fifth_pos), fifth_val);
    assert!(matches!(container.palette, ContainerPalette::Vector(_)));
    assert_eq!(container.bits, 3);

    // Check that all other values are 0.
    for index in 0..ChunkSection::VOLUME {
        let pos = SectionBlockPosition::from_index(index as usize);
        if pos != first_pos
            && pos != second_pos
            && pos != third_pos
            && pos != fourth_pos
            && pos != fifth_pos
        {
            assert_eq!(container.get_data(&pos), 0);
        }
    }
}

#[test]
fn wiki_example() {
    let container = BlockContainer {
        bits: 5,
        palette: ContainerPalette::Vector(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ]),
        data: BitVec::from_slice(&[
            //                                       ...{  4 }{  4 } {  3 }{  2 }{  2 }{  1 }
            0b0000_0000_0010_0000_1000_0110_0011_0001_0100_1000_0100_0001_1000_1000_0100_0001,
            //                                       ...{ 16 }{ 15 } { 13 }{  3 }{  4 }{  7 }
            0b0000_0001_0000_0001_1000_1010_0111_0010_0110_0000_1111_0110_1000_1100_1000_0111,
        ]),
        _phantom: std::marker::PhantomData,
    };

    for (i, n) in [1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 2u32]
        .into_iter()
        .enumerate()
    {
        assert_eq!(container.get_data(&SectionBlockPosition::from_index(i)), n);
    }
}

#[test]
fn global_container() {
    let mut container = BlockContainer::default();
    assert_eq!(container.bits, 0);

    // Fill the container with a bunch of values.
    for index in 0..ChunkSection::VOLUME {
        let pos = SectionBlockPosition::from_index(index as usize);

        assert_eq!(container.set_data(&pos, index), 0);
        assert_eq!(container.get_data(&pos), index);
    }

    // Check that the container is now a global palette.
    assert_eq!(container.palette, ContainerPalette::Global);
    assert_eq!(container.bits, 12);

    // Check that all values are still correct.
    for index in 0..ChunkSection::VOLUME {
        let pos = SectionBlockPosition::from_index(index as usize);

        assert_eq!(container.get_data(&pos), index);
    }
}

#[test]
#[cfg(feature = "froglight-block")]
fn chunk_blocks() {
    use bevy::{app::App, MinimalPlugins};
    use froglight_block::{
        attribute::SnowyBooleanAttribute,
        block::{Air, Blocks, GrassBlock, Stone},
        BlockPlugin, BlockStateExt, BlockStorageArc, VanillaResolver,
    };
    use froglight_protocol::{common::ChunkBlockPosition, versions::v1_21_0::V1_21_0};

    use crate::Chunk;

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(BlockPlugin);

    // Retrieve the block storage.
    let storage = app.world().resource::<BlockStorageArc<V1_21_0>>();
    let storage = storage.read();

    let chunk = Chunk::new_empty(16, 0);

    // All blocks in the chunk should be air.
    for blockstate_id in chunk.block_iter() {
        assert_eq!(storage.get_vanilla(blockstate_id), Some(Blocks::Air(Air)));
    }

    // Set the first half of the blocks to stone.
    let halfway = chunk.volume() / 2;
    for index in 0..halfway {
        let pos = ChunkBlockPosition::from_index(index as usize);
        chunk.set_block::<V1_21_0, VanillaResolver>(pos, &Stone, &storage);
    }

    // Set the next row to non-snowy grass.
    for index in halfway..halfway + 16 {
        let pos = ChunkBlockPosition::from_index(index as usize);
        chunk.set_block::<V1_21_0, VanillaResolver>(
            pos,
            &<GrassBlock as BlockStateExt<V1_21_0>>::from_attributes(SnowyBooleanAttribute(false)),
            &storage,
        );
    }

    // Check that all blocks are set correctly.
    for (index, blockstate_id) in chunk.block_iter().enumerate() {
        let expected = if index < halfway as usize {
            Blocks::Stone(Stone)
        } else if index < halfway as usize + 16 {
            Blocks::GrassBlock(BlockStateExt::<V1_21_0>::from_attributes(SnowyBooleanAttribute(
                false,
            )))
        } else {
            Blocks::Air(Air)
        };

        assert_eq!(storage.get_vanilla(blockstate_id), Some(expected));
    }
}
