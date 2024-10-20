//! @generated by `froglight-generator` #b0e1aa4

use froglight_macros::FrogRegistry;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum VillagerTypeKey {
    #[frog(key = "minecraft:desert")]
    Desert,
    #[frog(key = "minecraft:jungle")]
    Jungle,
    #[frog(key = "minecraft:plains")]
    #[default]
    Plains,
    #[frog(key = "minecraft:savanna")]
    Savanna,
    #[frog(key = "minecraft:snow")]
    Snow,
    #[frog(key = "minecraft:swamp")]
    Swamp,
    #[frog(key = "minecraft:taiga")]
    Taiga,
}
