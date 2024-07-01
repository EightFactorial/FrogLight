//! @generated by `froglight-generator` #8ccbfa2

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum WorldgenPoolAliasBindingKey {
    #[frog(key = "minecraft:random")]
    Random,
    #[frog(key = "minecraft:random_group")]
    RandomGroup,
    #[frog(key = "minecraft:direct")]
    Direct,
}
