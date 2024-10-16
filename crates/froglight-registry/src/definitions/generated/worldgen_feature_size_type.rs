//! @generated by `froglight-generator` #b0e1aa4

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum WorldgenFeatureSizeTypeKey {
    #[frog(key = "minecraft:two_layers_feature_size")]
    TwoLayersFeatureSize,
    #[frog(key = "minecraft:three_layers_feature_size")]
    ThreeLayersFeatureSize,
}
