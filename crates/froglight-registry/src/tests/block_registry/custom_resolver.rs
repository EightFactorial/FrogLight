use std::any::TypeId;

use compact_str::CompactString;
use froglight_protocol::common::ResourceKey;

use super::block_definitions::*;
use crate::{
    definitions::{
        BlockExt, BlockRegistry, BlockStateResolver, BlockStorage, BlockType, VanillaResolver,
    },
    tests::TestVersion,
};

/// A custom block that is not registered by the default resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SomeOtherBlock;

impl BlockType<TestVersion> for SomeOtherBlock {
    fn to_key(&self) -> ResourceKey { ResourceKey::new_inline("froglight:some_other") }
    fn to_lang(&self) -> CompactString { CompactString::from("block.froglight.some_other") }
}

impl BlockExt<TestVersion> for SomeOtherBlock {
    const BLOCK_STATES: u32 = 1u32;
    fn default_state() -> Self { SomeOtherBlock }
}

/// A custom block resolver that only resolves `SomeOtherBlock`.
///
/// All other blocks are ignored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SomeOtherResolver;

impl BlockStateResolver<TestVersion> for SomeOtherResolver {
    type Result = Option<SomeOtherBlock>;

    fn resolve(state_id: u32, storage: &BlockStorage<TestVersion>) -> Self::Result {
        let default_dyn = storage.get_default_dyn(state_id)?;
        if default_dyn.type_id() == TypeId::of::<SomeOtherBlock>() {
            Some(SomeOtherBlock)
        } else {
            None
        }
    }

    fn register_defaults(storage: &mut BlockStorage<TestVersion>) {
        storage.register::<SomeOtherBlock>();
    }
}

#[test]
fn custom_resolver() {
    let mut registry = BlockRegistry::<TestVersion>::default();

    // Check that only the default blocks are registered
    {
        assert_eq!(registry.get_block::<VanillaResolver>(0), Some(TestBlocks::Air(AirBlock)));
        assert_eq!(registry.get_block::<VanillaResolver>(1), Some(TestBlocks::Stone(StoneBlock)));
        assert_eq!(registry.get_block::<VanillaResolver>(4), None);

        // The new block should not be registered yet.
        assert_eq!(registry.get_block::<SomeOtherResolver>(4), None);
    }

    // Register blocks from `SomeOtherResolver`
    registry.register_defaults::<SomeOtherResolver>();

    // Check that the block is now registered
    {
        // The old resolver should be unaffected.
        assert_eq!(registry.get_block::<VanillaResolver>(0), Some(TestBlocks::Air(AirBlock)));
        assert_eq!(registry.get_block::<VanillaResolver>(1), Some(TestBlocks::Stone(StoneBlock)));
        assert_eq!(registry.get_block::<VanillaResolver>(4), None);

        // The new resolver should have registered the block.
        assert_eq!(registry.get_block::<SomeOtherResolver>(0), None);
        assert_eq!(registry.get_block::<SomeOtherResolver>(1), None);
        assert_eq!(registry.get_block::<SomeOtherResolver>(4), Some(SomeOtherBlock));
    }
}
