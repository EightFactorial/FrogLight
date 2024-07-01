//! @generated by `froglight-generator` #3701f00

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum LootPoolEntryTypeKey {
    #[frog(key = "minecraft:empty")]
    Empty,
    #[frog(key = "minecraft:item")]
    Item,
    #[frog(key = "minecraft:loot_table")]
    LootTable,
    #[frog(key = "minecraft:dynamic")]
    Dynamic,
    #[frog(key = "minecraft:tag")]
    Tag,
    #[frog(key = "minecraft:alternatives")]
    Alternatives,
    #[frog(key = "minecraft:sequence")]
    Sequence,
    #[frog(key = "minecraft:group")]
    Group,
}
