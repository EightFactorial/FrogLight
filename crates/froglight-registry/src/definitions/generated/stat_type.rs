//! @generated by `froglight-generator` #3701f00

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum StatTypeKey {
    #[frog(key = "minecraft:mined")]
    Mined,
    #[frog(key = "minecraft:crafted")]
    Crafted,
    #[frog(key = "minecraft:used")]
    Used,
    #[frog(key = "minecraft:broken")]
    Broken,
    #[frog(key = "minecraft:picked_up")]
    PickedUp,
    #[frog(key = "minecraft:dropped")]
    Dropped,
    #[frog(key = "minecraft:killed")]
    Killed,
    #[frog(key = "minecraft:killed_by")]
    KilledBy,
    #[frog(key = "minecraft:custom")]
    Custom,
}