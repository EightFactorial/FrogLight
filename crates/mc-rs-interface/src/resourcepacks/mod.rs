use bevy::prelude::*;

mod source;
use source::ResourcePackLoader;

mod asset;
pub use asset::*;

#[cfg(test)]
mod test;

/// This must be done *before* the AssetServer plugin is added.
pub(super) fn register_assets(app: &mut App) { source::register(app); }

/// This must be done *after* the AssetServer plugin is added.
pub(super) fn init_assets(app: &mut App) {
    app.init_asset::<ResourcePackAsset>()
        .init_asset_loader::<ResourcePackLoader>();
}
