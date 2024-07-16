//! @generated by `froglight-generator` #3f83759

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum HeightProviderTypeKey {
    #[frog(key = "minecraft:constant")]
    Constant,
    #[frog(key = "minecraft:uniform")]
    Uniform,
    #[frog(key = "minecraft:biased_to_bottom")]
    BiasedToBottom,
    #[frog(key = "minecraft:very_biased_to_bottom")]
    VeryBiasedToBottom,
    #[frog(key = "minecraft:trapezoid")]
    Trapezoid,
    #[frog(key = "minecraft:weighted_list")]
    WeightedList,
}
