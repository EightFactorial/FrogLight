use bevy::reflect::Reflect;
use froglight_assets::assets::ModelDefinition;
use froglight_network::common::ResourceKey;

mod block_model;
pub use block_model::*;

mod item_model;
use hashbrown::HashMap;
pub use item_model::*;

mod element;
pub use element::*;

/// A Model for a block or item
#[derive(Debug, Clone, PartialEq, Reflect)]
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
    ) -> Option<Self> {
        match def {
            ModelDefinition::Block(block) => {
                BlockModel::resolve_definition(key, block, definitions).map(Self::Block)
            }
            ModelDefinition::Item(item) => {
                ItemModel::resolve_definition(key, item, definitions).map(Self::Item)
            }
        }
    }
}
