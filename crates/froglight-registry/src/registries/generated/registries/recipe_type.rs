//! @generated by `froglight-generator` #3701f00

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum RecipeTypeKey {
    #[frog(key = "minecraft:crafting")]
    Crafting,
    #[frog(key = "minecraft:smelting")]
    Smelting,
    #[frog(key = "minecraft:blasting")]
    Blasting,
    #[frog(key = "minecraft:smoking")]
    Smoking,
    #[frog(key = "minecraft:campfire_cooking")]
    CampfireCooking,
    #[frog(key = "minecraft:stonecutting")]
    Stonecutting,
    #[frog(key = "minecraft:smithing")]
    Smithing,
}
