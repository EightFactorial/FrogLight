//! [`ResourcePack`](super::ResourcePack) asset loaders.

use bevy_app::App;
use bevy_asset::AssetApp;
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
    /// No metadata was found.
    #[error("No metadata found")]
    NoMetadata,
}
