//! [`ResourcePack`](super::ResourcePack) asset loaders.

use async_zip::ZipEntry;
use bevy_app::App;
use bevy_asset::AssetApp;
use froglight_common::ResourceKey;
use thiserror::Error;

mod folder;
pub use folder::ResourcePackLoader;

mod zip;
pub use zip::ResourcePackZipLoader;

/// Note: `ResourcePackZipLoader` **must** be
/// initialized last for embedded resource packs to work.
#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset_loader::<ResourcePackLoader>();
    app.init_asset_loader::<ResourcePackZipLoader>();
}

/// Errors that can occur when loading a [`ResourcePack`](super::ResourcePack).
#[derive(Debug, Error)]
pub enum ResourcePackLoaderError {
    /// An I/O error occurred.
    #[error("An I/O error occurred: {0}")]
    Io(#[from] std::io::Error),
    /// A JSON error occurred.
    #[error("A JSON error occurred: {0}")]
    Json(#[from] serde_json::Error),
    /// A ZIP error occurred.
    #[error("A ZIP error occurred: {0}")]
    Zip(#[from] async_zip::error::ZipError),
    /// An error occurred while loading an asset.
    #[error("An error occurred while loading an asset: {0}")]
    Direct(#[from] bevy_asset::LoadDirectError),
    /// No metadata was found.
    #[error("No metadata found")]
    NoMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
enum EntryType {
    BlockModel,
    BlockState,
    ItemModel,
    Language,
    ResourcePack,
    Sound,
    SoundMap,
    Texture,
    TextureAtlas,
}

impl EntryType {
    /// Get the [`EntryType`] and [`ResourceKey`] from a [`ZipEntry`].
    fn from_entry(entry: &ZipEntry) -> Option<(Self, ResourceKey)> {
        let path = entry.filename().as_str().ok()?;

        // Skip icons, directories, and .mcmeta files.
        if path.starts_with("assets/icons") || path.ends_with('/') || path.ends_with(".mcmeta") {
            return None;
        }

        let entry_type = Self::type_from_path(path)?;
        let entry_key = Self::key_from_path(path)?;

        Some((entry_type, entry_key))
    }

    /// Get the [`EntryType`] from a path.
    fn type_from_path(path: &str) -> Option<Self> {
        // Get the folder and file extension
        let folder = path.split('/').nth(2)?;
        let extension = path.split('.').last()?;

        // Match folders with expected file extensions
        match (folder, extension) {
            ("textures", "png") => Some(Self::Texture),
            ("sounds", "ogg") => Some(Self::Sound),
            ("blockstates", "json") => Some(Self::BlockState),
            ("models", "json") if path.contains("models/block") => Some(Self::BlockModel),
            ("models", "json") if path.contains("models/item") => Some(Self::ItemModel),
            ("lang", "json") => Some(Self::Language),
            ("atlases", "json") => Some(Self::TextureAtlas),
            ("resourcepacks", "zip") => Some(Self::ResourcePack),
            ("sounds.json", "json") => Some(Self::SoundMap),

            // Suppress warnings for known unsupported assets.
            #[cfg(debug_assertions)]
            ("font" | "particles" | "shaders" | "texts", _) => None,

            // Suppress warnings for known but unused assets.
            #[cfg(debug_assertions)]
            ("gpu_warnlist.json" | "regional_compliancies.json", _) => None,

            // Warn about unknown assets in debug mode.
            #[cfg(debug_assertions)]
            _ => {
                bevy_log::warn!("Unknown asset: \"{path}\"");
                None
            }
            // Ignore unknown assets in release mode.
            #[cfg(not(debug_assertions))]
            _ => None,
        }
    }

    /// Get the [`ResourceKey`] from a path.
    #[allow(clippy::single_match)]
    fn key_from_path(path: &str) -> Option<ResourceKey> {
        let mut split = path.split('/');
        let namespace = split.nth(1)?;

        // Exceptions for files not found in a folder.
        match split.next()? {
            "sounds.json" => {
                return ResourceKey::try_new(format!("{namespace}:sounds")).ok();
            }
            _ => {}
        }

        let mut path = split.remainder()?;
        path = path.split_once('.')?.0;

        ResourceKey::try_new(format!("{namespace}:{path}")).ok()
    }
}
