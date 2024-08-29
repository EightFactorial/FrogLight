use bevy_app::App;
use bevy_asset::{embedded_asset, AssetServer, Handle};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    prelude::ReflectResource,
    system::Resource,
    world::{FromWorld, World},
};
use bevy_reflect::Reflect;
use bevy_render::texture::{Image, ImageLoaderSettings, ImageSampler};
use froglight_common::ResourceKey;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    embedded_asset!(app, "fallback.png");

    app.register_type::<FallbackTexture>();
    app.init_resource::<FallbackTexture>();
}

/// A fallback texture used when a texture is missing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource, Reflect)]
#[reflect(Resource)]
pub struct FallbackTexture(Handle<Image>);

impl FallbackTexture {
    /// The path to the embedded fallback texture.
    pub const ASSET_PATH: &'static str =
        "embedded://froglight_asset/assets/processed/texture/fallback.png";

    /// The [`ResourceKey`] for the fallback texture.
    pub const ASSET_KEY: ResourceKey = ResourceKey::const_new("froglight:fallback");

    /// Returns a [`Handle`] to the fallback texture.
    #[must_use]
    pub fn as_handle(&self) -> &Handle<Image> { &self.0 }
}

impl AsRef<Handle<Image>> for FallbackTexture {
    fn as_ref(&self) -> &Handle<Image> { &self.0 }
}
impl AsMut<Handle<Image>> for FallbackTexture {
    fn as_mut(&mut self) -> &mut Handle<Image> { &mut self.0 }
}

impl FromWorld for FallbackTexture {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self(assets.load_with_settings(
            FallbackTexture::ASSET_PATH,
            |s: &mut ImageLoaderSettings| {
                s.sampler = ImageSampler::nearest();
            },
        ))
    }
}
