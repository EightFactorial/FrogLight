//! @generated by `froglight-generator` #b0e1aa4

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum ScheduleKey {
    #[frog(key = "minecraft:empty")]
    Empty,
    #[frog(key = "minecraft:simple")]
    Simple,
    #[frog(key = "minecraft:villager_baby")]
    VillagerBaby,
    #[frog(key = "minecraft:villager_default")]
    VillagerDefault,
}
