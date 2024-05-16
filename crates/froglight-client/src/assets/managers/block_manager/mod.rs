use bevy::prelude::*;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<BlockManager>().register_type::<BlockManager>();
}

/// A [`Resource`] for managing the blocks loaded in the game.
#[derive(Debug, Default, Clone, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct BlockManager {
    /// Loaded blocks.
    pub blocks: HashMap<ResourceKey, ()>,
}
