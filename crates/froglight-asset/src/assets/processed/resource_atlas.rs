use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    reflect::ReflectResource,
    system::{ResMut, Resource},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::texture::Image;
use bevy_sprite::TextureAtlasLayout;
use bevy_state::state::OnExit;

use crate::AssetState;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<ResourceAtlasStorage>();

    // Clear the `ResourceAtlasStorage` when assets are unloaded
    app.add_systems(OnExit(AssetState::Loaded), ResourceAtlasStorage::clear);

    // Register `ResourceAtlas`
    app.register_type::<ResourceAtlas>()
        .register_type::<Handle<ResourceAtlas>>()
        .register_type_data::<Handle<ResourceAtlas>, ReflectHandle>()
        .init_asset::<ResourceAtlas>();
}

/// An atlas of resource textures.
#[derive(Debug, Default, Clone, PartialEq, Reflect, Asset)]
#[reflect(Default, Asset)]
pub struct ResourceAtlas {
    /// A [`Handle`] to the atlas's [`Image`].
    pub atlas_image: Handle<Image>,
    /// A [`Handle`] to the atlas's [`TextureAtlasLayout`].
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

// --- Handle Storage ---

/// A [`Vec`] used to store [`Handle::Strong`] references to [`ResourceAtlas`]s.
///
/// Without this, [`ResourceAtlas`]s would unload when they are no longer in
/// use.
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Resource, Deref, DerefMut)]
#[reflect(Default, Resource)]
pub(crate) struct ResourceAtlasStorage {
    inner: Vec<Handle<ResourceAtlas>>,
}
impl ResourceAtlasStorage {
    /// Clear the [`ResourceAtlasStorage`].
    fn clear(mut res: ResMut<Self>) { res.clear() }
}
