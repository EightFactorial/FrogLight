use std::path::PathBuf;

use bevy_asset::{
    io::{AssetSource as BevyAssetSource, AssetSourceId as BevyAssetSourceId},
    AssetApp,
};
use bevy_ecs::system::Resource;
use bevy_log::info;

use crate::SettingsPlugin;

#[doc(hidden)]
pub(super) fn build(plugin: &SettingsPlugin, app: &mut bevy_app::App) {
    // Create a new ConfigFolder
    let folder = match plugin.path.as_ref() {
        Some(path) => ConfigFolder::new(path),
        None => ConfigFolder::from_env(),
    };

    // Optionally add an AssetSource
    if plugin.asset_source {
        let path_str =
            folder.path.to_str().expect("Unable to convert config folder path to string");

        info!("Loading files from: \"{path_str}\"");

        let reader = BevyAssetSource::get_default_reader(path_str.into());
        let writer = BevyAssetSource::get_default_writer(path_str.into());

        let source = BevyAssetSource::build().with_reader(reader).with_writer(writer);
        app.register_asset_source(BevyAssetSourceId::new(Some("frog")), source);
    }

    // Insert the ConfigFolder
    app.insert_resource(folder);
}

/// The configuration folder.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct ConfigFolder {
    /// The path to the configuration folder.
    pub path: PathBuf,
}

impl ConfigFolder {
    /// The default folder name for the asset source.
    pub const DEFAULT_FOLDER: &'static str = "FrogLight";

    /// Creates a new [`AssetSourcePlugin`] with the given path.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self { Self { path: path.into() } }

    /// Creates a new [`ConfigFolder`] with the default path based on
    /// environment variables.
    ///
    /// The default path of the `config` directory is based on the current
    /// platform and environment variables, with the folder name
    /// [`Self::DEFAULT_FOLDER`].
    ///
    /// # Panics
    /// - If the config directory cannot be found.
    #[must_use]
    pub fn from_env() -> Self {
        // Get the config directory
        let mut path = dirs::config_dir().expect("Unable to get config directory");
        assert!(path.exists(), "Config directory does not exist: `{}`", path.display());

        // Create the default folder if it doesn't exist
        path.push(Self::DEFAULT_FOLDER);
        if !path.exists() {
            info!("Creating new config directory: `{}`", path.display());
            if let Err(err) = std::fs::create_dir_all(&path) {
                panic!("Unable to create new config directory `{}`: `{err}`", path.display());
            }
        }

        Self::new(path)
    }
}
