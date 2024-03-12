use std::path::PathBuf;

use bevy_app::{App, Plugin};
use bevy_log::info;

/// Adds the [`AssetSource`](super::AssetSource) resource to the app
/// and adds the `frog://` asset source.
///
/// Must be added to the [`App`] before bevy's
/// [`AssetPlugin`](bevy_asset::AssetPlugin).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetSourcePlugin(PathBuf);

impl AssetSourcePlugin {
    /// The default folder name for the asset source.
    pub const DEFAULT_FOLDER: &'static str = "FrogLight";

    /// Creates a new [`AssetSourcePlugin`] with the given path.
    pub fn new(path: impl Into<PathBuf>) -> Self { Self(path.into()) }

    /// Creates a new [`AssetSourcePlugin`] with the default path based on
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

impl Default for AssetSourcePlugin {
    fn default() -> Self { Self::from_env() }
}

impl Plugin for AssetSourcePlugin {
    fn build(&self, app: &mut App) {
        // Add `SystemSet`s
        crate::systemset::build(app);

        // Add the asset source
        super::AssetSource::build(&self.0, app);
    }
}
