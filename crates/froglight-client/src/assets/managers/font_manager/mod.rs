use bevy::prelude::*;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<FontManager>().register_type::<FontManager>();
}

/// A [`Resource`] for managing font assets.
#[derive(Debug, Default, Clone, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct FontManager {
    /// Loaded fonts.
    pub fonts: HashMap<ResourceKey, Handle<Font>>,
}
