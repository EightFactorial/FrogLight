#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy_app::{PluginGroup, PluginGroupBuilder};

pub mod assets;
pub use assets::AssetDefinitionPlugin;

mod loaders;
pub use loaders::{
    AssetLoaderPlugin, ResourcePackFolderLoader, ResourcePackZipLoader, SerdeJsonLoader,
};

mod processor;
pub use processor::{AssetLoadState, AssetProcessorPlugin, AssetState};

mod storage;
pub use storage::{AssetKey, AssetStorage, AssetStoragePlugin};

/// A [`PluginGroup`] that contains all of the asset plugins.
///
/// Includes:
/// - [`AssetDefinitionPlugin`]
/// - [`AssetLoaderPlugin`]
/// - [`AssetProcessorPlugin`]
/// - [`AssetStoragePlugin`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetPlugins;

impl PluginGroup for AssetPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AssetDefinitionPlugin)
            .add(AssetLoaderPlugin)
            .add(AssetProcessorPlugin)
            .add(AssetStoragePlugin)
    }
}
