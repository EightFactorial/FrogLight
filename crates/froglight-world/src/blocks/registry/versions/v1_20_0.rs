use froglight_core::common::ResourceKey;
use froglight_protocol::versions::v1_20_0::V1_20_0;

use crate::blocks::{
    attributes::SnowyAttribute,
    block_list::{
        BlockAir, BlockAndesite, BlockCoarseDirt, BlockCobblestone, BlockDiorite, BlockDirt,
        BlockGranite, BlockGrassBlock, BlockPodzol, BlockPolishedAndesite, BlockPolishedDiorite,
        BlockPolishedGranite, BlockStone,
    },
    registry::InnerRegistry,
    traits::BlockRegistration,
    BlockType,
};

impl BlockRegistration for V1_20_0 {
    fn register_default(registry: &mut InnerRegistry<Self>) {
        registry
            .register_block(BlockAir)
            .register_block(BlockStone)
            .register_block(BlockGranite)
            .register_block(BlockPolishedGranite)
            .register_block(BlockDiorite)
            .register_block(BlockPolishedDiorite)
            .register_block(BlockAndesite)
            .register_block(BlockPolishedAndesite)
            .register_block(BlockGrassBlock::default())
            .register_block(BlockDirt)
            .register_block(BlockCoarseDirt)
            .register_block(BlockPodzol::default())
            .register_block(BlockCobblestone);
    }
}

impl BlockType<V1_20_0> for BlockAir {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:air") }
}

impl BlockType<V1_20_0> for BlockStone {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:stone") }
}

impl BlockType<V1_20_0> for BlockGranite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:granite") }
}

impl BlockType<V1_20_0> for BlockPolishedGranite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:polished_granite") }
}

impl BlockType<V1_20_0> for BlockDiorite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:diorite") }
}

impl BlockType<V1_20_0> for BlockPolishedDiorite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:polished_diorite") }
}

impl BlockType<V1_20_0> for BlockAndesite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:andesite") }
}

impl BlockType<V1_20_0> for BlockPolishedAndesite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:polished_andesite") }
}

impl BlockType<V1_20_0> for BlockGrassBlock {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:grass_block") }

    fn states(&self) -> u32 { 2 }

    fn from_relative_state(&self, id: usize) -> Option<Self>
    where
        Self: Sized + Default,
    {
        match id {
            0 => Some(Self { snowy: SnowyAttribute(false) }),
            1 => Some(Self { snowy: SnowyAttribute(true) }),
            _ => None,
        }
    }
}

impl BlockType<V1_20_0> for BlockDirt {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:dirt") }
}

impl BlockType<V1_20_0> for BlockCoarseDirt {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:coarse_dirt") }
}

impl BlockType<V1_20_0> for BlockPodzol {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:podzol") }

    fn states(&self) -> u32 { 2 }

    fn from_relative_state(&self, id: usize) -> Option<Self>
    where
        Self: Sized + Default,
    {
        match id {
            0 => Some(Self { snowy: SnowyAttribute(false) }),
            1 => Some(Self { snowy: SnowyAttribute(true) }),
            _ => None,
        }
    }
}

impl BlockType<V1_20_0> for BlockCobblestone {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:cobblestone") }
}
