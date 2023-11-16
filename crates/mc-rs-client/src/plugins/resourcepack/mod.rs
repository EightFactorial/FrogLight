//! ResourcePack source plugin

use std::time::Duration;

use bevy::{
    asset::io::{AssetSource, AssetSourceId},
    prelude::*,
};

use crate::dir::config_folder;

#[cfg(test)]
mod test;

/// A plugin that adds the `resourcepack://` asset source.
///
/// This plugin is automatically added to the client plugin group,
/// but can be added manually if needed.
///
/// Defaults to:
/// - Linux: `~/.config/MC-RS/resourcepacks`
/// - Windows: `%APPDATA%/MC-RS/resourcepacks`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ResourcePackSourcePlugin;

impl Plugin for ResourcePackSourcePlugin {
    fn build(&self, app: &mut App) {
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
            debug!("Loading ResourcePacks from {:?}", path);

            let watcher = AssetSource::get_default_watcher(path.into(), Duration::from_secs(15));
            source = source.with_watcher(watcher);
        }

        app.register_asset_source(AssetSourceId::Name("resourcepack".into()), source);
    }
}
