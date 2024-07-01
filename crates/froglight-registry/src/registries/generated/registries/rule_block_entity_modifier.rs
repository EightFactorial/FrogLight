//! @generated by `froglight-generator` #8ccbfa2

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum RuleBlockEntityModifierKey {
    #[frog(key = "minecraft:clear")]
    Clear,
    #[frog(key = "minecraft:passthrough")]
    Passthrough,
    #[frog(key = "minecraft:append_static")]
    AppendStatic,
    #[frog(key = "minecraft:append_loot")]
    AppendLoot,
}
