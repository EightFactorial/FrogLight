//! [`ResourcePackMeta`](super::ResourcePackMeta) asset loaders.

use bevy_app::App;
use bevy_asset::AssetApp;

mod folder;
pub use folder::ResourcePackMetaLoader;

mod zip;
pub use zip::ResourcePackMetaZipLoader;

/// Note: `ResourcePackMetaLoader` **must** be initialized last.
#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset_loader::<ResourcePackMetaZipLoader>();
    app.init_asset_loader::<ResourcePackMetaLoader>();
}
