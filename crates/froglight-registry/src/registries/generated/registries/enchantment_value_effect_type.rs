//! @generated by `froglight-generator` #3701f00

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum EnchantmentValueEffectTypeKey {
    #[frog(key = "minecraft:add")]
    Add,
    #[frog(key = "minecraft:all_of")]
    AllOf,
    #[frog(key = "minecraft:multiply")]
    Multiply,
    #[frog(key = "minecraft:remove_binomial")]
    RemoveBinomial,
    #[frog(key = "minecraft:set")]
    Set,
}
