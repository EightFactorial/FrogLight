//! @generated by `froglight-generator` #3701f00

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum FluidKey {
    #[frog(key = "minecraft:empty")]
    #[default]
    Empty,
    #[frog(key = "minecraft:flowing_water")]
    FlowingWater,
    #[frog(key = "minecraft:water")]
    Water,
    #[frog(key = "minecraft:flowing_lava")]
    FlowingLava,
    #[frog(key = "minecraft:lava")]
    Lava,
}
