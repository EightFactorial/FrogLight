use bevy::{
    app::{App, Plugin},
    asset::AssetApp,
};

pub(crate) mod asset;
pub use asset::{
    meta::{ResourcePackMeta, ResourcePackMetaContainer},
    model, ResourcePackAsset,
};

pub(crate) mod loader;
pub use loader::{ResourcePackLoader, ResourcePackLoaderError};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ResourcePackPlugin;

impl Plugin for ResourcePackPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ResourcePackAsset>()
            .init_asset_loader::<ResourcePackLoader>();
    }
}
