//! @generated by `froglight-generator` #248246d

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum LootNbtProviderTypeKey {
    #[frog(key = "minecraft:storage")]
    Storage,
    #[frog(key = "minecraft:context")]
    Context,
}
