//! @generated by `froglight-generator` #cd8324b

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum WorldgenChunkGeneratorKey {
    #[frog(key = "minecraft:noise")]
    Noise,
    #[frog(key = "minecraft:flat")]
    Flat,
    #[frog(key = "minecraft:debug")]
    Debug,
}
