use bevy_app::{App, Plugin};

mod pack;
use bevy_asset::AssetApp;
pub use pack::{ResourcePackFolderLoader, ResourcePackZipLoader};

mod serde;
pub use serde::SerdeJsonLoader;

/// A [`Plugin`] that registers all of the asset loaders.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<ResourcePackZipLoader>()
            .init_asset_loader::<ResourcePackFolderLoader>();

        // TODO: Init loaders for each type loaded by `SerdeJsonLoader`.
    }
}
