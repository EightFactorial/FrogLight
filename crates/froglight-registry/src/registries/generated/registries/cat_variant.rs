//! @generated by `froglight-generator` #3701f00

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum CatVariantKey {
    #[frog(key = "minecraft:tabby")]
    Tabby,
    #[frog(key = "minecraft:black")]
    Black,
    #[frog(key = "minecraft:red")]
    Red,
    #[frog(key = "minecraft:siamese")]
    Siamese,
    #[frog(key = "minecraft:british_shorthair")]
    BritishShorthair,
    #[frog(key = "minecraft:calico")]
    Calico,
    #[frog(key = "minecraft:persian")]
    Persian,
    #[frog(key = "minecraft:ragdoll")]
    Ragdoll,
    #[frog(key = "minecraft:white")]
    White,
    #[frog(key = "minecraft:jellie")]
    Jellie,
    #[frog(key = "minecraft:all_black")]
    AllBlack,
}
