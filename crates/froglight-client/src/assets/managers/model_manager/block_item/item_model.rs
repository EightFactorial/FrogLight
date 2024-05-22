use bevy::prelude::*;
use froglight_assets::assets::{ItemModelDefinition, ModelDefinition};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

/// An Item Model
#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub struct ItemModel {}

impl ItemModel {
    /// Resolves an [`ItemModelDefinition`] into an [`ItemModel`].
    #[must_use]
    pub fn resolve_definition(
        _key: &ResourceKey,
        _def: &ItemModelDefinition,
        _definitions: &HashMap<ResourceKey, ModelDefinition>,
    ) -> Self {
        Self {}
    }
}
