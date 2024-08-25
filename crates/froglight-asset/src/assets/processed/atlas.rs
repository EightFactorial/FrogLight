use std::sync::Arc;

use bevy_app::App;
use bevy_asset::Handle;
use bevy_derive::Deref;
use bevy_ecs::{reflect::ReflectResource, system::Resource};
use bevy_reflect::Reflect;
use bevy_render::texture::Image;
use bevy_sprite::TextureAtlasLayout;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<BlockAtlas>(); }

/// The block texture atlas.
///
/// This is safe to clone and share across threads.
#[derive(Debug, Clone, Deref, Resource, Reflect)]
#[reflect(Resource)]
pub struct BlockAtlas(Arc<(TextureAtlasLayout, Handle<TextureAtlasLayout>, Handle<Image>)>);

impl BlockAtlas {
    /// Creates a new block texture atlas.
    #[must_use]
    pub fn new(
        layout: TextureAtlasLayout,
        atlas: Handle<TextureAtlasLayout>,
        image: Handle<Image>,
    ) -> Self {
        Self(Arc::new((layout, atlas, image)))
    }

    /// Returns the layout of the [`BlockAtlas`].
    #[must_use]
    pub fn layout(&self) -> &TextureAtlasLayout { &self.0 .0 }

    /// Returns the handle to the layout of the [`BlockAtlas`].
    #[must_use]
    pub fn layout_handle(&self) -> &Handle<TextureAtlasLayout> { &self.0 .1 }

    /// Returns the handle to the image of the [`BlockAtlas`].
    #[must_use]
    pub fn image_handle(&self) -> &Handle<Image> { &self.0 .2 }
}
