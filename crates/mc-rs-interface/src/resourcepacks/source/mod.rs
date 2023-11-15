use std::time::Duration;

use bevy::{
    asset::io::{AssetSource, AssetSourceId},
    prelude::*,
};

use crate::util::dir::config_folder;

mod loaders;
pub use loaders::ResourcePackLoader;

/// Adds the resourcepack asset source to the app.
///
/// This must be done *before* the AssetServer plugin is added.
///
/// ### Example
/// ```rust,no_run
/// use bevy::prelude::*;
/// use mc_rs_interface::resourcepacks::ResourcePackAsset;
///
/// fn load_pack(assets: Res<AssetServer>) {
///     let _pack: Handle<ResourcePackAsset> = assets.load("resourcepack://minecraft.zip");
/// }
/// ```
pub(super) fn register(app: &mut App) {
    let path = config_folder().join("resourcepacks");
    let Some(path) = path.to_str() else {
        panic!("Invalid resourcepack path: {:?}", path);
    };

    let reader = AssetSource::get_default_reader(path.into());
    let writer = AssetSource::get_default_writer(path.into());

    let mut source = AssetSource::build().with_reader(reader).with_writer(writer);

    // Add a watcher to the resourcepack asset source in debug mode.
    #[cfg(any(debug_assertions, feature = "debug"))]
    {
        debug!("ResourcePack path: {:?}", path);

        let watcher = AssetSource::get_default_watcher(path.into(), Duration::from_secs(15));
        source = source.with_watcher(watcher);
    }

    app.register_asset_source(AssetSourceId::Name("resourcepack".into()), source);
}
