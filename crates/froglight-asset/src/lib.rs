#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_id)]
#![feature(str_split_remainder)]

use bevy_app::{PluginGroup, PluginGroupBuilder};

pub mod assets;
pub use assets::{AssetPlugin, ResourcePack, ResourcePackMeta};

pub mod catalog;
pub use catalog::{AssetCatalog, AssetKey, CatalogPlugin};

mod source;
pub use source::AssetSourcePlugin;

pub mod processor;
pub use processor::{
    AssetProcess, AssetProcessorPlugin, AssetState, ResourceLoadTrigger, ResourcePackList,
    ResourceResetTrigger,
};

/// A [`PluginGroup`] that contains all of the asset plugins.
///
/// Includes:
/// - [`AssetPlugin`]
/// - [`AssetSourcePlugin`]
/// - [`AssetProcessorPlugin`]
/// - [`CatalogPlugin`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetPlugins;

impl PluginGroup for AssetPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AssetPlugin)
            .add(AssetSourcePlugin::default())
            .add(CatalogPlugin)
            .add(AssetProcessorPlugin)
    }
}
