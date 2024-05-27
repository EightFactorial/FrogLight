use bevy::prelude::*;
use froglight_assets::assets::{
    model::{GuiLight, ModelDisplayTransform},
    ItemModelDefinition, ModelDefinition,
};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

use super::ItemModelOverrides;
use crate::assets::AssetManager;

/// An Item Model
#[derive(Debug, Reflect)]
pub struct ItemModel {
    /// The gui light for the model
    pub gui_light: GuiLight,

    /// Item model mesh
    ///
    /// This is used for rendering the item in the player's hand,
    /// in the inventory, in item frames, etc.
    pub item_model: Handle<Mesh>,

    /// Overrides for when alternative models should be used
    pub overrides: ItemModelOverrides,

    /// Model transforms
    ///
    /// Indexed via
    /// [`DisplayPosition`](froglight_assets::assets::model::DisplayPosition).
    pub model_transforms: [ModelDisplayTransform; 7],
}

impl ItemModel {
    /// Resolves an [`ItemModelDefinition`] into an [`ItemModel`].
    #[must_use]
    pub fn resolve_definition(
        _key: &ResourceKey,
        _def: &ItemModelDefinition,
        _definitions: &HashMap<ResourceKey, ModelDefinition>,
        _asset_manager: &AssetManager,
        _mesh_assets: &mut Assets<Mesh>,
    ) -> Self {
        Self {
            model_transforms: Default::default(),
            gui_light: GuiLight::Front,
            item_model: Handle::default(),
            overrides: ItemModelOverrides::default(),
        }
    }
}
