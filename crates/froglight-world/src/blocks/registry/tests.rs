use bevy_ecs::world::{FromWorld, World};
use froglight_core::common::ChunkBlockPosition;
use froglight_protocol::versions::v1_20_0::V1_20_0;

use super::BlockRegistry;
use crate::{
    blocks::{
        attributes::SnowyAttribute,
        block_list::{BlockAir, BlockEnum, BlockGrassBlock, BlockPodzol, BlockStone},
    },
    world::Chunk,
};

#[test]
fn empty_chunk() {
    let chunk = crate::world::Chunk::new_empty(384, -64);

    let registry = BlockRegistry::<V1_20_0>::from_world(&mut World::default());
    let registry = registry.read();

    for block_id in chunk.block_iter() {
        if let Some(dyn_block) = registry.get_block(block_id) {
            let block = BlockEnum::from_dyn(dyn_block, block_id, &registry);

            // All blocks should be air
            assert_eq!(block, Some(BlockEnum::Air(BlockAir)));
        } else {
            panic!("Block `{block_id}` not found in registry");
        }
    }
}

#[test]
fn full_chunk() {
    let mut chunk = crate::world::Chunk::new_empty(384, -64);

    // Fill the chunk with stone
    for index in 0..chunk.height * Chunk::DEPTH * Chunk::WIDTH {
        chunk.set_blockid(&ChunkBlockPosition::from_index(index), 1);
    }

    // Get the block registry
    let registry = BlockRegistry::<V1_20_0>::from_world(&mut World::default());
    let registry = registry.read();

    // Check that all blocks are stone
    for block_id in chunk.block_iter() {
        if let Some(dyn_block) = registry.get_block(block_id) {
            let block = BlockEnum::from_dyn(dyn_block, block_id, &registry);

            // All blocks should be stone
            assert_eq!(block, Some(BlockEnum::Stone(BlockStone)));
        } else {
            panic!("Block `{block_id}` not found in registry");
        }
    }
}

#[test]
fn snowy_grass() {
    let mut chunk = crate::world::Chunk::new_empty(384, -64);

    // Insert grass blocks, alternating the snowy attribute
    for index in 0..chunk.height * Chunk::DEPTH * Chunk::WIDTH {
        let value = u32::try_from(8 + (index % 2)).unwrap();
        chunk.set_blockid(&ChunkBlockPosition::from_index(index), value);
    }

    // Get the block registry
    let registry = BlockRegistry::<V1_20_0>::from_world(&mut World::default());
    let registry = registry.read();

    for (index, block_id) in chunk.block_iter().enumerate() {
        if let Some(dyn_block) = registry.get_block(block_id) {
            // Get the block, with state information
            let block = BlockEnum::from_dyn(dyn_block, block_id, &registry).unwrap();

            if index % 2 == 0 {
                // Even blocks are not snowy
                assert_eq!(
                    block,
                    BlockEnum::GrassBlock(BlockGrassBlock { snowy: SnowyAttribute(false) })
                );
            } else {
                // Odd blocks are snowy
                assert_eq!(
                    block,
                    BlockEnum::GrassBlock(BlockGrassBlock { snowy: SnowyAttribute(true) })
                );
            }
        } else {
            panic!("Block `{block_id}` not found in registry");
        }
    }
}

#[test]
fn reverse_lookup() {
    let registry = BlockRegistry::<V1_20_0>::from_world(&mut World::default());
    let registry = registry.read();

    // Get the block index for the air block
    {
        let block_range =
            registry.type_range::<BlockAir>().expect("BlockAir not found in registry");

        let dyn_air =
            registry.get_block(block_range.start).expect("BlockAir not found in registry");
        assert_eq!(dyn_air.resource_key(), "minecraft:air");
    }

    // Get the block index for the stone block
    {
        let block_range =
            registry.type_range::<BlockStone>().expect("BlockStone not found in registry");

        let dyn_stone =
            registry.get_block(block_range.start).expect("BlockStone not found in registry");
        assert_eq!(dyn_stone.resource_key(), "minecraft:stone");
    }

    // Get the block index for the grass block
    {
        let block_range = registry
            .type_range::<BlockGrassBlock>()
            .expect("BlockGrassBlock not found in registry");

        // The range should contain at least two blocks
        assert_ne!(block_range.start, block_range.end - 1);

        // The first block should be the non-snowy grass block
        let dyn_grass =
            registry.get_block(block_range.start).expect("BlockGrassBlock not found in registry");
        assert_eq!(dyn_grass.resource_key(), "minecraft:grass_block");

        // The second block should be the snowy grass block
        let dyn_grass_snowy =
            registry.get_block(block_range.end - 1).expect("BlockGrassBlock not found in registry");
        assert_eq!(dyn_grass_snowy.resource_key(), "minecraft:grass_block");
    }

    // Get the block index for the podzol block
    {
        let block_range =
            registry.type_range::<BlockPodzol>().expect("BlockPodzol not found in registry");

        // The range should contain at least two blocks
        assert_ne!(block_range.start, block_range.end - 1);

        // The first block should be the non-snowy podzol block
        let dyn_podzol =
            registry.get_block(block_range.start).expect("BlockPodzol not found in registry");
        assert_eq!(dyn_podzol.resource_key(), "minecraft:podzol");

        // The second block should be the snowy podzol block
        let dyn_podzol_snowy =
            registry.get_block(block_range.end - 1).expect("BlockPodzol not found in registry");
        assert_eq!(dyn_podzol_snowy.resource_key(), "minecraft:podzol");
    }
}
