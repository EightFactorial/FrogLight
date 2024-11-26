//! Generated tests for [`V1_21_0`].
//!
//! @generated by 'TODO'
#![allow(clippy::wildcard_imports)]
use std::any::TypeId;

use bevy::MinimalPlugins;
use bevy_app::App;
use froglight_protocol::versions::v1_21_0::V1_21_0;

use crate::{block::*, BlockPlugin, BlockStorageArc};

#[test]
#[expect(clippy::too_many_lines)]
fn generated() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(BlockPlugin);

    // Retrieve the block storage.
    let storage = app.world().resource::<BlockStorageArc<V1_21_0>>();
    let storage = storage.read();

    if let Some(block) = storage.get_stored_default(18596u32) {
        assert_eq!(block.resource_key(), "minecraft:crimson_stem");
        assert_eq!(block.type_id(), TypeId::of::<CrimsonStem>());

        let downcast = block.as_any().downcast_ref::<CrimsonStem>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(18597u32));
    }
    if let Some(block) = storage.get_stored_default(18404u32) {
        assert_eq!(block.resource_key(), "minecraft:loom");
        assert_eq!(block.type_id(), TypeId::of::<Loom>());

        let downcast = block.as_any().downcast_ref::<Loom>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(18404u32));
    }
    if let Some(block) = storage.get_stored_default(2049u32) {
        assert_eq!(block.resource_key(), "minecraft:magenta_wool");
        assert_eq!(block.type_id(), TypeId::of::<MagentaWool>());

        let downcast = block.as_any().downcast_ref::<MagentaWool>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(2049u32));
    }
    if let Some(block) = storage.get_stored_default(25399u32) {
        assert_eq!(block.resource_key(), "minecraft:polished_deepslate_slab");
        assert_eq!(block.type_id(), TypeId::of::<PolishedDeepslateSlab>());

        let downcast = block.as_any().downcast_ref::<PolishedDeepslateSlab>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(25402u32));
    }
    if let Some(block) = storage.get_stored_default(23116u32) {
        assert_eq!(block.resource_key(), "minecraft:exposed_cut_copper_stairs");
        assert_eq!(block.type_id(), TypeId::of::<ExposedCutCopperStairs>());

        let downcast = block.as_any().downcast_ref::<ExposedCutCopperStairs>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(23127u32));
    }
    if let Some(block) = storage.get_stored_default(521u32) {
        assert_eq!(block.resource_key(), "minecraft:deepslate_lapis_ore");
        assert_eq!(block.type_id(), TypeId::of::<DeepslateLapisOre>());

        let downcast = block.as_any().downcast_ref::<DeepslateLapisOre>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(521u32));
    }
    if let Some(block) = storage.get_stored_default(21001u32) {
        assert_eq!(block.resource_key(), "minecraft:orange_candle_cake");
        assert_eq!(block.type_id(), TypeId::of::<OrangeCandleCake>());

        let downcast = block.as_any().downcast_ref::<OrangeCandleCake>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(21002u32));
    }
    if let Some(block) = storage.get_stored_default(7666u32) {
        assert_eq!(block.resource_key(), "minecraft:spruce_stairs");
        assert_eq!(block.type_id(), TypeId::of::<SpruceStairs>());

        let downcast = block.as_any().downcast_ref::<SpruceStairs>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(7677u32));
    }
    if let Some(block) = storage.get_stored_default(18u32) {
        assert_eq!(block.resource_key(), "minecraft:jungle_planks");
        assert_eq!(block.type_id(), TypeId::of::<JunglePlanks>());

        let downcast = block.as_any().downcast_ref::<JunglePlanks>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(18u32));
    }
    if let Some(block) = storage.get_stored_default(23844u32) {
        assert_eq!(block.resource_key(), "minecraft:weathered_copper_door");
        assert_eq!(block.type_id(), TypeId::of::<WeatheredCopperDoor>());

        let downcast = block.as_any().downcast_ref::<WeatheredCopperDoor>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(23855u32));
    }
    if let Some(block) = storage.get_stored_default(10751u32) {
        assert_eq!(block.resource_key(), "minecraft:rose_bush");
        assert_eq!(block.type_id(), TypeId::of::<RoseBush>());

        let downcast = block.as_any().downcast_ref::<RoseBush>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(10752u32));
    }
    if let Some(block) = storage.get_stored_default(23304u32) {
        assert_eq!(block.resource_key(), "minecraft:waxed_oxidized_cut_copper");
        assert_eq!(block.type_id(), TypeId::of::<WaxedOxidizedCutCopper>());

        let downcast = block.as_any().downcast_ref::<WaxedOxidizedCutCopper>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(23304u32));
    }
    if let Some(block) = storage.get_stored_default(23288u32) {
        assert_eq!(block.resource_key(), "minecraft:exposed_cut_copper_slab");
        assert_eq!(block.type_id(), TypeId::of::<ExposedCutCopperSlab>());

        let downcast = block.as_any().downcast_ref::<ExposedCutCopperSlab>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(23291u32));
    }
    if let Some(block) = storage.get_stored_default(156u32) {
        assert_eq!(block.resource_key(), "minecraft:muddy_mangrove_roots");
        assert_eq!(block.type_id(), TypeId::of::<MuddyMangroveRoots>());

        let downcast = block.as_any().downcast_ref::<MuddyMangroveRoots>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(157u32));
    }
    if let Some(block) = storage.get_stored_default(5218u32) {
        assert_eq!(block.resource_key(), "minecraft:dark_oak_hanging_sign");
        assert_eq!(block.type_id(), TypeId::of::<DarkOakHangingSign>());

        let downcast = block.as_any().downcast_ref::<DarkOakHangingSign>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(5251u32));
    }
    if let Some(block) = storage.get_stored_default(22952u32) {
        assert_eq!(block.resource_key(), "minecraft:waxed_oxidized_chiseled_copper");
        assert_eq!(block.type_id(), TypeId::of::<WaxedOxidizedChiseledCopper>());

        let downcast = block.as_any().downcast_ref::<WaxedOxidizedChiseledCopper>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(22952u32));
    }
    if let Some(block) = storage.get_stored_default(7272u32) {
        assert_eq!(block.resource_key(), "minecraft:nether_bricks");
        assert_eq!(block.type_id(), TypeId::of::<NetherBricks>());

        let downcast = block.as_any().downcast_ref::<NetherBricks>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(7272u32));
    }
    if let Some(block) = storage.get_stored_default(198u32) {
        assert_eq!(block.resource_key(), "minecraft:jungle_wood");
        assert_eq!(block.type_id(), TypeId::of::<JungleWood>());

        let downcast = block.as_any().downcast_ref::<JungleWood>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(199u32));
    }
    if let Some(block) = storage.get_stored_default(11019u32) {
        assert_eq!(block.resource_key(), "minecraft:orange_wall_banner");
        assert_eq!(block.type_id(), TypeId::of::<OrangeWallBanner>());

        let downcast = block.as_any().downcast_ref::<OrangeWallBanner>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(11019u32));
    }
    if let Some(block) = storage.get_stored_default(5782u32) {
        assert_eq!(block.resource_key(), "minecraft:cactus");
        assert_eq!(block.type_id(), TypeId::of::<Cactus>());

        let downcast = block.as_any().downcast_ref::<Cactus>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(5782u32));
    }
    if let Some(block) = storage.get_stored_default(13442u32) {
        assert_eq!(block.resource_key(), "minecraft:stone_stairs");
        assert_eq!(block.type_id(), TypeId::of::<StoneStairs>());

        let downcast = block.as_any().downcast_ref::<StoneStairs>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(13453u32));
    }
    if let Some(block) = storage.get_stored_default(12640u32) {
        assert_eq!(block.resource_key(), "minecraft:brown_shulker_box");
        assert_eq!(block.type_id(), TypeId::of::<BrownShulkerBox>());

        let downcast = block.as_any().downcast_ref::<BrownShulkerBox>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(12644u32));
    }
    if let Some(block) = storage.get_stored_default(26221u32) {
        assert_eq!(block.resource_key(), "minecraft:deepslate_brick_slab");
        assert_eq!(block.type_id(), TypeId::of::<DeepslateBrickSlab>());

        let downcast = block.as_any().downcast_ref::<DeepslateBrickSlab>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(26224u32));
    }
    if let Some(block) = storage.get_stored_default(5866u32) {
        assert_eq!(block.resource_key(), "minecraft:carved_pumpkin");
        assert_eq!(block.type_id(), TypeId::of::<CarvedPumpkin>());

        let downcast = block.as_any().downcast_ref::<CarvedPumpkin>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(5866u32));
    }
    if let Some(block) = storage.get_stored_default(24164u32) {
        assert_eq!(block.resource_key(), "minecraft:copper_trapdoor");
        assert_eq!(block.type_id(), TypeId::of::<CopperTrapdoor>());

        let downcast = block.as_any().downcast_ref::<CopperTrapdoor>().unwrap();
        assert_eq!(storage.get_block_id(downcast), Some(24179u32));
    }

}