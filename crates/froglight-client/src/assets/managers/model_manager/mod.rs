//! [`ModelManager`]
//!
//! Holds models for blocks, items, and entities.

use std::sync::Arc;

use bevy::prelude::*;
use froglight_assets::assets::ModelDefinition as BlockItemDefinition;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;
use parking_lot::RwLock;

mod block_item;
pub use block_item::*;

mod entity;
pub use entity::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<ModelManager>()
        .register_type::<ModelManager>()
        .init_resource::<ModelManagerState>()
        .register_type::<ModelManagerState>();
}

/// A [`Resource`] for managing model assets.
#[derive(Debug, Default, Clone, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct ModelManager {
    /// Block and Item Definitions
    #[reflect(ignore)]
    _block_item_defs: HashMap<ResourceKey, BlockItemDefinition>,
    /// Block and Item Models
    #[reflect(ignore)]
    pub block_item: BlockItemModels,

    /// Entity Models
    pub entities: HashMap<ResourceKey, EntityModel>,
}

/// Block and Item models stored in a [`HashMap`].
pub type BlockItemModels = Arc<RwLock<HashMap<ResourceKey, BlockItemModel>>>;

/// A [`Resource`] for managing the state of the [`ModelManager`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Reflect)]
#[reflect(Default, Resource)]
struct ModelManagerState {
    finished: bool,
    current_pack: usize,
    current_model: usize,
}
