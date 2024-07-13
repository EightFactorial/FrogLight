//! @generated by `froglight-generator` #cd8324b

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum WorldgenFeatureSizeTypeKey {
    #[frog(key = "minecraft:two_layers_feature_size")]
    TwoLayersFeatureSize,
    #[frog(key = "minecraft:three_layers_feature_size")]
    ThreeLayersFeatureSize,
}
