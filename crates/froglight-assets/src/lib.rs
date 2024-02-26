#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod plugin;

#[cfg(feature = "asset_manager")]
mod asset_manager;
#[cfg(feature = "asset_manager")]
pub use asset_manager::{manager::AssetManager, tracker::AssetTracker};

mod asset_source;
pub use asset_source::AssetSource;

mod resourcepack;
pub use resourcepack::{
    asset::{
        meta::{AssetMcMeta, PackMcMeta},
        ResourcePack,
    },
    fallback::FallbackImage,
    loader::{ResourcePackLoader, ResourcePackLoaderError},
};

mod settings;
pub use settings::resourcepack_config::ResourcePackSettings;

mod systemset;
pub use systemset::resourcepack::ResourcePackState;
