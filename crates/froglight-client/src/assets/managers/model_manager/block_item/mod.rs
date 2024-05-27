use asset_manager::AssetManager;
use bevy::prelude::*;
use froglight_assets::assets::ModelDefinition;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

mod block;
pub use block::*;

mod item;
pub use item::*;

mod element;
pub use element::*;

use crate::assets::managers::asset_manager;

/// A Model for a block or item
#[derive(Debug, Reflect)]
#[allow(clippy::large_enum_variant)]
pub enum BlockItemModel {
    /// A block model
    Block(BlockModel),
    /// An item model
    Item(ItemModel),
}

impl BlockItemModel {
    /// Resolves a [`ModelDefinition`] into a [`BlockItemModel`].
    #[must_use]
    pub fn resolve_definition(
        key: &ResourceKey,
        def: &ModelDefinition,
        definitions: &HashMap<ResourceKey, ModelDefinition>,
        asset_manager: &AssetManager,
        mesh_assets: &mut Assets<Mesh>,
    ) -> Self {
        match def {
            ModelDefinition::Block(block) => Self::Block(BlockModel::resolve_definition(
                key,
                block,
                definitions,
                asset_manager,
                mesh_assets,
            )),
            ModelDefinition::Item(item) => Self::Item(ItemModel::resolve_definition(
                key,
                item,
                definitions,
                asset_manager,
                mesh_assets,
            )),
        }
    }
}
