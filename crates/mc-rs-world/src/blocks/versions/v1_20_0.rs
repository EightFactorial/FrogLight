use crate::blocks::{structs::*, traits::Block};
use mc_rs_protocol::versions::v1_20_0::V1_20_0;

impl Block<V1_20_0> for BlockError {
    fn resource_location(&self) -> &'static str { "mc-rs:error" }
    fn state_id(&self) -> u32 { u32::MAX }
}

impl Block<V1_20_0> for BlockAir {
    fn resource_location(&self) -> &'static str { "minecraft:air" }
    fn state_id(&self) -> u32 { 0 }
}

impl Block<V1_20_0> for BlockStone {
    fn resource_location(&self) -> &'static str { "minecraft:stone" }
    fn state_id(&self) -> u32 { 1 }
}

impl Block<V1_20_0> for BlockGranite {
    fn resource_location(&self) -> &'static str { "minecraft:granite" }
    fn state_id(&self) -> u32 { 2 }
}

impl Block<V1_20_0> for BlockPolishedGranite {
    fn resource_location(&self) -> &'static str { "minecraft:polished_granite" }
    fn state_id(&self) -> u32 { 3 }
}

impl Block<V1_20_0> for BlockDiorite {
    fn resource_location(&self) -> &'static str { "minecraft:diorite" }
    fn state_id(&self) -> u32 { 4 }
}

impl Block<V1_20_0> for BlockPolishedDiorite {
    fn resource_location(&self) -> &'static str { "minecraft:polished_diorite" }
    fn state_id(&self) -> u32 { 5 }
}

impl Block<V1_20_0> for BlockAndesite {
    fn resource_location(&self) -> &'static str { "minecraft:andesite" }
    fn state_id(&self) -> u32 { 6 }
}

impl Block<V1_20_0> for BlockPolishedAndesite {
    fn resource_location(&self) -> &'static str { "minecraft:polished_andesite" }
    fn state_id(&self) -> u32 { 7 }
}

impl Block<V1_20_0> for BlockGrassBlock {
    fn resource_location(&self) -> &'static str { "minecraft:grass_block" }
    fn state_id(&self) -> u32 { 8 }
}

impl Block<V1_20_0> for BlockDirt {
    fn resource_location(&self) -> &'static str { "minecraft:dirt" }
    fn state_id(&self) -> u32 { 9 }
}
