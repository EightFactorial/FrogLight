//! @generated by `froglight-generator` #3f83759

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum WorldgenChunkGeneratorKey {
    #[frog(key = "minecraft:noise")]
    Noise,
    #[frog(key = "minecraft:flat")]
    Flat,
    #[frog(key = "minecraft:debug")]
    Debug,
}
