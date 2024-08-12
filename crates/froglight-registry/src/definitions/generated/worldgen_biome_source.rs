//! @generated by `froglight-generator` #b0e1aa4

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum WorldgenBiomeSourceKey {
    #[frog(key = "minecraft:fixed")]
    Fixed,
    #[frog(key = "minecraft:multi_noise")]
    MultiNoise,
    #[frog(key = "minecraft:checkerboard")]
    Checkerboard,
    #[frog(key = "minecraft:the_end")]
    TheEnd,
}
