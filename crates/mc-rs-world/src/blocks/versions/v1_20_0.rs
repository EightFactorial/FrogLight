use crate::blocks::{
    attributes::SnowyAttribute,
    structs::*,
    traits::{BlockTrait, BlocksTrait},
    Blocks,
};
use mc_rs_protocol::versions::v1_20_0::V1_20_0;

impl BlocksTrait<V1_20_0> for Blocks {
    fn from_u32(id: u32) -> Self {
        match id {
            0 => Self::Air(BlockAir::try_from_u32(id).unwrap()),
            1 => Self::Stone(BlockStone::try_from_u32(id).unwrap()),
            2 => Self::Granite(BlockGranite::try_from_u32(id).unwrap()),
            3 => Self::PolishedGranite(BlockPolishedGranite::try_from_u32(id).unwrap()),
            4 => Self::Diorite(BlockDiorite::try_from_u32(id).unwrap()),
            5 => Self::PolishedDiorite(BlockPolishedDiorite::try_from_u32(id).unwrap()),
            6 => Self::Andesite(BlockAndesite::try_from_u32(id).unwrap()),
            7 => Self::PolishedAndesite(BlockPolishedAndesite::try_from_u32(id).unwrap()),
            8..=9 => Self::GrassBlock(BlockGrassBlock::try_from_u32(id).unwrap()),
            10 => Self::Dirt(BlockDirt::try_from_u32(id).unwrap()),
            _ => Self::Error(BlockError),
        }
    }
    fn to_u32(&self) -> u32 { todo!() }
}

impl BlockTrait<V1_20_0> for BlockError {
    fn resource_location(&self) -> &'static str { "mc-rs:error" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { u32::MAX }
}

impl BlockTrait<V1_20_0> for BlockAir {
    fn resource_location(&self) -> &'static str { "minecraft:air" }
    fn try_from_u32(id: u32) -> Option<Self> {
        if id == 0 {
            Some(Self)
        } else {
            None
        }
    }
    fn to_u32(&self) -> u32 { 0 }
}

impl BlockTrait<V1_20_0> for BlockStone {
    fn resource_location(&self) -> &'static str { "minecraft:stone" }
    fn try_from_u32(id: u32) -> Option<Self> {
        if id == 1 {
            Some(Self)
        } else {
            None
        }
    }
    fn to_u32(&self) -> u32 { 1 }
}

impl BlockTrait<V1_20_0> for BlockGranite {
    fn resource_location(&self) -> &'static str { "minecraft:granite" }
    fn try_from_u32(id: u32) -> Option<Self> {
        if id == 2 {
            Some(Self)
        } else {
            None
        }
    }
    fn to_u32(&self) -> u32 { 2 }
}

impl BlockTrait<V1_20_0> for BlockPolishedGranite {
    fn resource_location(&self) -> &'static str { "minecraft:polished_granite" }
    fn try_from_u32(id: u32) -> Option<Self> {
        if id == 3 {
            Some(Self)
        } else {
            None
        }
    }
    fn to_u32(&self) -> u32 { 3 }
}

impl BlockTrait<V1_20_0> for BlockDiorite {
    fn resource_location(&self) -> &'static str { "minecraft:diorite" }
    fn try_from_u32(id: u32) -> Option<Self> {
        if id == 4 {
            Some(Self)
        } else {
            None
        }
    }
    fn to_u32(&self) -> u32 { 4 }
}

impl BlockTrait<V1_20_0> for BlockPolishedDiorite {
    fn resource_location(&self) -> &'static str { "minecraft:polished_diorite" }
    fn try_from_u32(id: u32) -> Option<Self> {
        if id == 5 {
            Some(Self)
        } else {
            None
        }
    }
    fn to_u32(&self) -> u32 { 5 }
}

impl BlockTrait<V1_20_0> for BlockAndesite {
    fn resource_location(&self) -> &'static str { "minecraft:andesite" }
    fn try_from_u32(id: u32) -> Option<Self> {
        if id == 6 {
            Some(Self)
        } else {
            None
        }
    }
    fn to_u32(&self) -> u32 { 6 }
}

impl BlockTrait<V1_20_0> for BlockPolishedAndesite {
    fn resource_location(&self) -> &'static str { "minecraft:polished_andesite" }
    fn try_from_u32(id: u32) -> Option<Self> {
        if id == 7 {
            Some(Self)
        } else {
            None
        }
    }
    fn to_u32(&self) -> u32 { 7 }
}

impl BlockTrait<V1_20_0> for BlockGrassBlock {
    fn resource_location(&self) -> &'static str { "minecraft:grass_block" }
    fn try_from_u32(id: u32) -> Option<Self> {
        match id {
            8 => Some(Self {
                snowy: SnowyAttribute(false),
            }),
            9 => Some(Self {
                snowy: SnowyAttribute(true),
            }),
            _ => None,
        }
    }
    fn to_u32(&self) -> u32 {
        if self.snowy.0 {
            9
        } else {
            8
        }
    }
}

impl BlockTrait<V1_20_0> for BlockDirt {
    fn resource_location(&self) -> &'static str { "minecraft:dirt" }
    fn try_from_u32(id: u32) -> Option<Self> {
        if id == 10 {
            Some(Self)
        } else {
            None
        }
    }
    fn to_u32(&self) -> u32 { 10 }
}
