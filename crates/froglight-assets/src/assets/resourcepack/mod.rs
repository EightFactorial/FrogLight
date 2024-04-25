use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle};
use bevy_audio::AudioSource;
use bevy_reflect::TypePath;
use bevy_render::texture::Image;
use froglight_protocol::common::ResourceKey;
use hashbrown::HashMap;

mod info;
pub use info::*;

mod loader;
pub use loader::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset::<ResourcePack>().init_asset_loader::<ResourcePackZipLoader>();
}

/// A bundle of assets.
#[derive(Debug, Default, Clone, PartialEq, Eq, Asset, TypePath)]
pub struct ResourcePack {
    /// [`ResourcePack`] information
    pub info: ResourcePackInfo,

    /// Textures
    pub textures: HashMap<ResourceKey, Handle<Image>>,
    /// Audio
    pub audio: HashMap<ResourceKey, Handle<AudioSource>>,

    /// Json files
    pub json: HashMap<ResourceKey, serde_json::Value>,
}
