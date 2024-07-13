//! @generated by `froglight-generator` #cd8324b

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum LootNumberProviderTypeKey {
    #[frog(key = "minecraft:constant")]
    Constant,
    #[frog(key = "minecraft:uniform")]
    Uniform,
    #[frog(key = "minecraft:binomial")]
    Binomial,
    #[frog(key = "minecraft:score")]
    Score,
    #[frog(key = "minecraft:storage")]
    Storage,
    #[frog(key = "minecraft:enchantment_level")]
    EnchantmentLevel,
}
