//! @generated by `froglight-generator` #cd8324b

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum LootScoreProviderTypeKey {
    #[frog(key = "minecraft:fixed")]
    Fixed,
    #[frog(key = "minecraft:context")]
    Context,
}
