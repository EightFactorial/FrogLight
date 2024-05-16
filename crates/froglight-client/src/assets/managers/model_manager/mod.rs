use bevy::prelude::*;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<ModelManager>().register_type::<ModelManager>();
}

/// A [`Resource`] for managing model assets.
#[derive(Debug, Default, Clone, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct ModelManager {
    /// Models
    pub models: HashMap<ResourceKey, ()>,
}
