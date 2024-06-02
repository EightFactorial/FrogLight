use std::any::TypeId;

use super::block_definitions::*;
use crate::{
    definitions::{BlockExt, BlockRegistry, BlockStateResolver, BlockStorage, VanillaResolver},
    tests::TestVersion,
};

impl BlockStateResolver<TestVersion> for VanillaResolver {
    type Result = Option<TestBlocks>;

    fn resolve(state_id: u32, storage: &BlockStorage<TestVersion>) -> Self::Result {
        let default_dyn = storage.get_default_dyn(state_id)?;
        match default_dyn.type_id() {
            id if id == TypeId::of::<AirBlock>() => Some(TestBlocks::Air(AirBlock)),
            id if id == TypeId::of::<StoneBlock>() => Some(TestBlocks::Stone(StoneBlock)),
            id if id == TypeId::of::<GrassBlock>() => {
                let relative_id = storage.relative_state_of(default_dyn, state_id)?;
                Some(TestBlocks::Grass(GrassBlock::from_relative_id(relative_id)?))
            }
            _ => None,
        }
    }

    fn register_defaults(storage: &mut BlockStorage<TestVersion>) {
        storage.register::<AirBlock>().register::<StoneBlock>().register::<GrassBlock>();
    }
}

#[test]
fn default_dyn() {
    let default = BlockRegistry::<TestVersion>::default();
    let read = default.read();

    // Check that the default air block is correct
    {
        let dyn_air = read.get_default_dyn(0).unwrap();
        let dyn_air = dyn_air.as_any().downcast_ref::<AirBlock>().unwrap();

        assert_eq!(dyn_air, &AirBlock::default_state());
    }

    // Check that the default stone block is correct
    {
        let dyn_stone = read.get_default_dyn(1).unwrap();
        let dyn_stone = dyn_stone.as_any().downcast_ref::<StoneBlock>().unwrap();

        assert_eq!(dyn_stone, &StoneBlock::default_state());
    }

    // Check that the default grass block is correct
    {
        let dyn_grass_bare = read.get_default_dyn(2).unwrap();
        let dyn_grass_bare = dyn_grass_bare.as_any().downcast_ref::<GrassBlock>().unwrap();

        // This is as expected, because the first state
        // is the default state for the grass block.
        assert_eq!(dyn_grass_bare, &GrassBlock { grassy: GrassyAttribute(false) });

        let dyn_grass_grassy = read.get_default_dyn(3).unwrap();
        let dyn_grass_grassy = dyn_grass_grassy.as_any().downcast_ref::<GrassBlock>().unwrap();

        // This is not correct, because the second state
        // is *not* the same as the default state.
        //
        // If you need to know the attributes for sure,
        // use `get_block` instead of `get_default_dyn`.
        assert_eq!(dyn_grass_grassy, &GrassBlock { grassy: GrassyAttribute(false) });
    }
}

#[test]
fn default_registry() {
    let default = BlockRegistry::<TestVersion>::default();

    // Check that all default blocks are registered
    assert_eq!(default.get_block::<VanillaResolver>(0), Some(TestBlocks::Air(AirBlock)));
    assert_eq!(default.get_block::<VanillaResolver>(1), Some(TestBlocks::Stone(StoneBlock)));

    // Check that the grass block has both states
    assert_eq!(
        default.get_block::<VanillaResolver>(2),
        Some(TestBlocks::Grass(GrassBlock { grassy: GrassyAttribute(false) }))
    );
    assert_eq!(
        default.get_block::<VanillaResolver>(3),
        Some(TestBlocks::Grass(GrassBlock { grassy: GrassyAttribute(true) }))
    );

    // Check that ids out of range return None
    assert_eq!(default.get_block::<VanillaResolver>(4), None);
}
