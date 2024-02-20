use std::path::{Path, PathBuf};

use bevy::{
    asset::io::{AssetSource as BevyAssetSource, AssetSourceId as BevyAssetSourceId},
    prelude::*,
};

pub(crate) mod plugin;

/// The path to the asset source directory.
///
/// Assets loaded using `frog://` will be loaded from this directory.
///
/// # Note
/// This is a read-only [`Resource`], and should not be modified at runtime.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, Resource, Reflect)]
#[reflect(Resource)]
pub struct AssetSource(PathBuf);

impl AssetSource {
    fn build(path: &Path, app: &mut App) {
        // Register and insert the `AssetSource` resource
        app.register_type::<AssetSource>().insert_resource(AssetSource(path.into()));

        // Create an asset reader and writer
        let path = path.to_str().unwrap();
        let reader = BevyAssetSource::get_default_reader(path.into());
        let writer = BevyAssetSource::get_default_writer(path.into());

        // Create and register the asset source
        let source = BevyAssetSource::build().with_reader(reader).with_writer(writer);
        app.register_asset_source(BevyAssetSourceId::Name("frog".into()), source);
    }
}
