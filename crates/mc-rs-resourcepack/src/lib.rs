use bevy::prelude::*;

mod asset;
pub use asset::ResourcePackAsset;
use loader::ResourcePackLoader;

mod loader;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ResourcePackPlugin;

impl Plugin for ResourcePackPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ResourcePackAsset>()
            .init_asset_loader::<ResourcePackLoader>();
    }
}
