use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use froglight_components::resourcekey::ResourceKey;
use serde::{Deserialize, Serialize};

use super::{ModelDisplayTransforms, ModelElement, ModelTextures};

/// A block model definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]

pub struct BlockModelDefinition {
    /// The parent model
    pub parent: Option<ResourceKey>,

    /// Whether to enable ambient occlusion, or use the parent's values
    #[serde(rename = "ambientocclusion", default, skip_serializing_if = "Option::is_none")]
    pub ambient_occlusion: Option<bool>,

    /// The display settings for the model, or use the parent's values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ModelDisplayTransforms>,

    /// The textures for the model, or use the parent's values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub textures: Option<ModelTextures>,

    /// The elements of the model, or use the parent's values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<ModelElement>>,
}
