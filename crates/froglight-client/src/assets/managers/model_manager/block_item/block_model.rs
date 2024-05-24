use bevy::prelude::*;
use froglight_assets::assets::{
    model::ModelDisplayTransform, BlockModelDefinition, ModelDefinition,
};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

use super::ModelElement;

/// An Item Model
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct BlockModel {
    /// Whether to enable ambient occlusion
    pub ambient_occlusion: bool,

    /// Model transforms
    ///
    /// Indexed via
    /// [`DisplayPosition`](froglight_assets::assets::model::DisplayPosition).
    pub model_transforms: [ModelDisplayTransform; 7],

    /// Model elements
    pub elements: Vec<ModelElement>,
}

impl BlockModel {
    /// Resolves a [`BlockModelDefinition`] into a [`BlockModel`].
    #[must_use]
    pub fn resolve_definition(
        _key: &ResourceKey,
        _def: &BlockModelDefinition,
        _definitions: &HashMap<ResourceKey, ModelDefinition>,
    ) -> Option<Self> {
        todo!()
    }
}
