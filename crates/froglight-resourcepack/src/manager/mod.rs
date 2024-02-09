use std::sync::Arc;

use bevy::{asset::AssetPath, prelude::*, utils::HashMap};
use froglight_core::data::ResourceKey;
use parking_lot::RwLock;

use crate::{
    settings::{ResourcePackAudioSettings, ResourcePackLoaderSettings},
    ResourcePack,
};

/// A manager for resource packs and their assets.
///
/// In order to load and track assets, clone this resource and create a
/// [`ResourcePackLoaderSettings`](crate::ResourcePackLoaderSettings) with it.
/// Then use [`AssetServer::load_with_settings`] to load a resource pack.
#[derive(Debug, Default, Clone, Resource)]
pub struct ResourcePackManager {
    /// A collection of loaded texture assets.
    pub texture_assets: Arc<RwLock<HashMap<ResourceKey, Handle<Image>>>>,

    /// The audio settings set by the resource packs.
    pub audio_settings: Arc<RwLock<Option<ResourcePackAudioSettings>>>,

    /// A collection of loaded audio assets.
    pub audio_assets: Arc<RwLock<HashMap<ResourceKey, Handle<AudioSource>>>>,
}

impl ResourcePackManager {
    /// Loads a resource pack from the given path.
    ///
    /// This is a convenience method for loading a resource pack with the
    /// [`ResourcePackLoaderSettings`] and [`AssetServer`].
    pub fn load_resourcepack<'a>(
        &self,
        path: impl Into<AssetPath<'a>>,
        asset_server: &AssetServer,
    ) -> Handle<ResourcePack> {
        asset_server.load_with_settings(path, Self::settings)
    }

    /// An empty settings function for loading resource packs.
    fn settings(_: &mut ResourcePackLoaderSettings) {}
}
