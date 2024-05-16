use bevy::prelude::*;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<AtlasManager>().register_type::<AtlasManager>();
}

/// A [`Resource`] for managing [`TextureAtlasLayout`] assets.
#[derive(Debug, Default, Clone, Resource, Reflect)]
pub struct AtlasManager {
    /// Texture Atlases
    pub atlases: HashMap<ResourceKey, Handle<TextureAtlasLayout>>,
}
