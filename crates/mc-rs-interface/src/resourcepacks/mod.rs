use bevy::prelude::*;

mod loader;
use loader::ResourcePackLoader;

mod source;

mod pack;
pub use pack::*;

/// This must be done *before* the AssetServer plugin is added.
pub(super) fn register_assets(app: &mut App) { source::register(app); }

/// This must be done *after* the AssetServer plugin is added.
pub(super) fn init_assets(app: &mut App) {
    app.init_asset::<ResourcePackAsset>()
        .init_asset_loader::<ResourcePackLoader>();
}
