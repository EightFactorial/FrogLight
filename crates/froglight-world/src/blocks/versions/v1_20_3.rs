use froglight_core::common::ResourceKey;
use froglight_protocol::versions::v1_20_3::V1_20_3;

use crate::blocks::{
    attributes::SnowyAttribute,
    blocks::{
        BlockAir, BlockAndesite, BlockCoarseDirt, BlockCobblestone, BlockDiorite, BlockDirt,
        BlockGranite, BlockGrassBlock, BlockPodzol, BlockPolishedAndesite, BlockPolishedDiorite,
        BlockPolishedGranite, BlockStone,
    },
    registry::BlockRegistryInner,
    BlockType,
};

#[doc(hidden)]
pub(crate) fn register(registry: &mut BlockRegistryInner<V1_20_3>) {
    // Register the vanilla blocks
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
        .register_block(BlockGrassBlock::default())
        .register_block(BlockDirt)
        .register_block(BlockCoarseDirt)
        .register_block(BlockPodzol::default())
        .register_block(BlockCobblestone);
}

impl BlockType<V1_20_3> for BlockAir {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:air") }

    fn from_relative_state(&self, _: usize) -> Option<Self> { Some(BlockAir) }

    fn is_air(&self) -> bool { true }
    fn is_opaque(&self) -> bool { false }
    fn is_collidable(&self) -> bool { false }
}

impl BlockType<V1_20_3> for BlockStone {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:stone") }

    fn from_relative_state(&self, _: usize) -> Option<Self> { Some(BlockStone) }
}

impl BlockType<V1_20_3> for BlockGranite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:granite") }

    fn from_relative_state(&self, _: usize) -> Option<Self> { Some(BlockGranite) }
}

impl BlockType<V1_20_3> for BlockPolishedGranite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:polished_granite") }

    fn from_relative_state(&self, _: usize) -> Option<Self> { Some(BlockPolishedGranite) }
}

impl BlockType<V1_20_3> for BlockDiorite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:diorite") }

    fn from_relative_state(&self, _: usize) -> Option<Self> { Some(BlockDiorite) }
}

impl BlockType<V1_20_3> for BlockPolishedDiorite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:polished_diorite") }

    fn from_relative_state(&self, _: usize) -> Option<Self> { Some(BlockPolishedDiorite) }
}

impl BlockType<V1_20_3> for BlockAndesite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:andesite") }

    fn from_relative_state(&self, _: usize) -> Option<Self> { Some(BlockAndesite) }
}

impl BlockType<V1_20_3> for BlockPolishedAndesite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:polished_andesite") }

    fn from_relative_state(&self, _: usize) -> Option<Self> { Some(BlockPolishedAndesite) }
}

impl BlockType<V1_20_3> for BlockGrassBlock {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:grass_block") }
    fn states(&self) -> usize { 2 }

    fn from_relative_state(&self, id: usize) -> Option<Self> {
        match id {
            0 => Some(BlockGrassBlock { snowy: SnowyAttribute(false) }),
            1 => Some(BlockGrassBlock { snowy: SnowyAttribute(true) }),
            _ => None,
        }
    }
}

impl BlockType<V1_20_3> for BlockDirt {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:dirt") }

    fn from_relative_state(&self, _: usize) -> Option<Self> { Some(BlockDirt) }
}

impl BlockType<V1_20_3> for BlockCoarseDirt {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:coarse_dirt") }

    fn from_relative_state(&self, _: usize) -> Option<Self> { Some(BlockCoarseDirt) }
}

impl BlockType<V1_20_3> for BlockPodzol {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:podzol") }
    fn states(&self) -> usize { 2 }

    fn from_relative_state(&self, id: usize) -> Option<Self> {
        match id {
            0 => Some(BlockPodzol { snowy: SnowyAttribute(false) }),
            1 => Some(BlockPodzol { snowy: SnowyAttribute(true) }),
            _ => None,
        }
    }
}

impl BlockType<V1_20_3> for BlockCobblestone {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:cobblestone") }

    fn from_relative_state(&self, _: usize) -> Option<Self> { Some(BlockCobblestone) }
}
