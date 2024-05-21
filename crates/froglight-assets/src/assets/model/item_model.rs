use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use froglight_components::resourcekey::ResourceKey;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use super::{ModelDisplayTransforms, ModelElement, ModelTextures};

/// A item model definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct ItemModelDefinition {
    /// The parent model
    pub parent: Option<ResourceKey>,

    /// The display settings for the model, or use the parent's values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ModelDisplayTransforms>,

    /// The textures for the model, or use the parent's values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub textures: Option<ModelTextures>,

    /// The gui light for the model, or use the parent's values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gui_light: Option<GuiLight>,

    /// The elements of the model, or use the parent's values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<ModelElement>>,

    /// Overrides for when alternative models should be used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<ItemModelOverride>>,
}

/// The gui light for the model
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GuiLight {
    /// Render the model flat
    Front,
    #[default]
    /// Render the model from the side, like a block
    ///
    /// This is the default
    Side,
}

/// An item model override
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct ItemModelOverride {
    /// The predicate for when to use this model
    #[reflect(ignore)]
    pub predicate: HashMap<String, serde_json::Value>,
    /// The model to use
    pub model: ResourceKey,
}
