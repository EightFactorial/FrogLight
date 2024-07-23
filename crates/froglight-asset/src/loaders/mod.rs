//! Asset loaders for loading [`ResourcePack`](super::assets::ResourcePack)s and
//! other assets.

use bevy_app::{App, Plugin};
use bevy_asset::AssetApp;

mod pack;
pub use pack::{
    ResourcePackFolderError, ResourcePackFolderLoader, ResourcePackZipError, ResourcePackZipLoader,
};

mod serde;
pub use serde::SerdeJsonLoader;

use crate::assets::unprocessed::NamespaceSoundMap;

/// A [`Plugin`] that registers all of the asset loaders.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<ResourcePackZipLoader>()
            .init_asset_loader::<ResourcePackFolderLoader>();

        // TODO: Init loaders for each type loaded by `SerdeJsonLoader`.
        app.init_asset_loader::<SerdeJsonLoader<NamespaceSoundMap>>();
    }
}
