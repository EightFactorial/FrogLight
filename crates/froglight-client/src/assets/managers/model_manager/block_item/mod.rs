use bevy::prelude::*;
use froglight_assets::assets::ModelDefinition;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

mod block_model;
pub use block_model::*;

mod item_model;
pub use item_model::*;

mod element;
pub use element::*;

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
        mesh_assets: &mut Assets<Mesh>,
    ) -> Self {
        match def {
            ModelDefinition::Block(block) => {
                Self::Block(BlockModel::resolve_definition(key, block, definitions, mesh_assets))
            }
            ModelDefinition::Item(item) => {
                Self::Item(ItemModel::resolve_definition(key, item, definitions, mesh_assets))
            }
        }
    }
}
