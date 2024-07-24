#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(str_split_remainder)]

use bevy_app::{PluginGroup, PluginGroupBuilder};

pub mod assets;
pub use assets::{AssetDefinitionPlugin, ResourcePack, ResourcePackMeta};

mod catalog;
pub use catalog::{AssetCatalog, AssetCatalogPlugin, AssetKey};

pub mod loaders;
pub use loaders::AssetLoaderPlugin;

mod processor;
pub use processor::*;

mod source;
pub use source::AssetSourcePlugin;

/// A [`PluginGroup`] that contains all of the asset plugins.
///
/// Includes:
/// - [`AssetSourcePlugin`]
/// - [`AssetDefinitionPlugin`]
/// - [`AssetLoaderPlugin`]
/// - [`AssetProcessorPlugin`]
/// - [`AssetStoragePlugin`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetPlugins;

impl PluginGroup for AssetPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AssetSourcePlugin::default())
            .add(AssetDefinitionPlugin)
            .add(AssetLoaderPlugin)
            .add(AssetProcessorPlugin)
            .add(AssetCatalogPlugin)
    }
}
