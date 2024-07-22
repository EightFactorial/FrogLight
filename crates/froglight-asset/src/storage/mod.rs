use bevy_app::{App, Plugin};
use bevy_render::{mesh::Mesh, texture::Image};

mod asset_access;
pub use asset_access::AssetKey;

mod asset_storage;
pub use asset_storage::AssetStorage;

/// A [`Plugin`] that registers the asset storage system.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetStoragePlugin;

impl Plugin for AssetStoragePlugin {
    fn build(&self, app: &mut App) {
        // Register and initialize the AssetStorage
        app.register_type::<AssetStorage>().init_resource::<AssetStorage>();

        // Register various AssetKey types
        app.register_type::<AssetKey<Image>>().register_type::<AssetKey<Mesh>>();
    }
}
