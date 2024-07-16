//! @generated by `froglight-generator` #3f83759

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
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
