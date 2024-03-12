use froglight_core::common::ResourceKey;
use froglight_macros::frog_version_blocks;
use froglight_protocol::versions::v1_20_0::V1_20_0;

use crate::blocks::{
    attributes::SnowyAttribute,
    block_list::{
        BlockAir, BlockAndesite, BlockCoarseDirt, BlockCobblestone, BlockDiorite, BlockDirt,
        BlockGranite, BlockGrassBlock, BlockPodzol, BlockPolishedAndesite, BlockPolishedDiorite,
        BlockPolishedGranite, BlockStone,
    },
    traits::{BlockExt, BlockRegistration},
    BlockType,
};

frog_version_blocks! {
    V1_20_0,
    Air,
    Stone,
    Granite,
    PolishedGranite,
    Diorite,
    PolishedDiorite,
    Andesite,
    PolishedAndesite,
    GrassBlock,
    Dirt,
    CoarseDirt,
    Podzol,
    Cobblestone,
}

impl BlockType<V1_20_0> for BlockAir {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:air") }

    fn is_air(&self) -> bool { true }
    fn is_opaque(&self) -> bool { false }
    fn is_collidable(&self) -> bool { false }
}
impl BlockExt<V1_20_0> for BlockAir {}

impl BlockType<V1_20_0> for BlockStone {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:stone") }
}
impl BlockExt<V1_20_0> for BlockStone {}

impl BlockType<V1_20_0> for BlockGranite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:granite") }
}
impl BlockExt<V1_20_0> for BlockGranite {}

impl BlockType<V1_20_0> for BlockPolishedGranite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:polished_granite") }
}
impl BlockExt<V1_20_0> for BlockPolishedGranite {}

impl BlockType<V1_20_0> for BlockDiorite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:diorite") }
}
impl BlockExt<V1_20_0> for BlockDiorite {}

impl BlockType<V1_20_0> for BlockPolishedDiorite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:polished_diorite") }
}
impl BlockExt<V1_20_0> for BlockPolishedDiorite {}

impl BlockType<V1_20_0> for BlockAndesite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:andesite") }
}
impl BlockExt<V1_20_0> for BlockAndesite {}

impl BlockType<V1_20_0> for BlockPolishedAndesite {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:polished_andesite") }
}
impl BlockExt<V1_20_0> for BlockPolishedAndesite {}

impl BlockType<V1_20_0> for BlockGrassBlock {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:grass_block") }

    fn states(&self) -> u32 { 2 }
}
impl BlockExt<V1_20_0> for BlockGrassBlock {
    fn from_relative_state(state: u32) -> Option<Self> {
        match state {
            0 => Some(Self { snowy: SnowyAttribute(false) }),
            1 => Some(Self { snowy: SnowyAttribute(true) }),
            _ => None,
        }
    }

    fn to_relative_state(&self) -> u32 { u32::from(self.snowy.0) }
}

impl BlockType<V1_20_0> for BlockDirt {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:dirt") }
}
impl BlockExt<V1_20_0> for BlockDirt {}

impl BlockType<V1_20_0> for BlockCoarseDirt {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:coarse_dirt") }
}
impl BlockExt<V1_20_0> for BlockCoarseDirt {}

impl BlockType<V1_20_0> for BlockPodzol {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:podzol") }

    fn states(&self) -> u32 { 2 }
}
impl BlockExt<V1_20_0> for BlockPodzol {
    fn from_relative_state(state: u32) -> Option<Self> {
        match state {
            0 => Some(Self { snowy: SnowyAttribute(false) }),
            1 => Some(Self { snowy: SnowyAttribute(true) }),
            _ => None,
        }
    }

    fn to_relative_state(&self) -> u32 { u32::from(self.snowy.0) }
}

impl BlockType<V1_20_0> for BlockCobblestone {
    fn resource_key(&self) -> ResourceKey { ResourceKey::new("minecraft:cobblestone") }
}
impl BlockExt<V1_20_0> for BlockCobblestone {}
