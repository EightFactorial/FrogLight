//! The [`ResourcePack`] asset and loader.

use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle};
use bevy_audio::AudioSource;
use bevy_reflect::TypePath;

mod loader;
use bevy_render::texture::Image;
use froglight_protocol::common::ResourceKey;
use hashbrown::HashMap;
pub use loader::{ResourcePackError, ResourcePackLoader};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the ResourcePack asset and loader.
    app.init_asset::<ResourcePack>().init_asset_loader::<ResourcePackLoader>();
}

/// A collection of assets and resource definitions.
#[derive(Debug, Default, Clone, PartialEq, Eq, Asset, TypePath)]
pub struct ResourcePack {
    /// Handles to all loaded textures.
    ///
    /// ### Note
    /// All handles are [`Weak`](Handle::Weak),
    /// [`Strong`](Handle::Strong) handles are stored in the
    /// [`AssetManager`](crate::asset_manager::AssetManager).
    pub textures: HashMap<ResourceKey, Handle<Image>>,

    /// Handles to all loaded sounds.
    ///
    /// ### Note
    /// All handles are [`Weak`](Handle::Weak),
    /// [`Strong`](Handle::Strong) handles are stored in the
    /// [`AssetManager`](crate::asset_manager::AssetManager).
    pub sounds: HashMap<ResourceKey, Handle<AudioSource>>,
}
