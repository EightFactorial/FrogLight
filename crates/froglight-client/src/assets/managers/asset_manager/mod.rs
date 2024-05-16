use bevy::prelude::*;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<AssetManager>().register_type::<AssetManager>();
}

/// A [`Resource`] for managing [`sound`](AudioSource) and [`texture`](Image)
/// assets.
#[derive(Debug, Default, Clone, Resource, Reflect)]
pub struct AssetManager {
    /// Sounds
    pub sounds: HashMap<ResourceKey, Handle<AudioSource>>,
    /// Textures
    pub textures: HashMap<ResourceKey, Handle<Image>>,
}
