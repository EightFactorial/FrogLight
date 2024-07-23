use std::path::PathBuf;

use bevy_app::{App, Plugin};
use bevy_asset::{io::AssetSourceBuilder, AssetApp};
use bevy_log::{error, warn};

/// A [`Plugin`] that registers an asset source.
///
/// Allows assets to be loaded from the source directory
/// using `{ source id }://{ asset path }`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetSourcePlugin {
    /// The source id.
    pub source_id: String,
    /// The source path.
    pub source_path: PathBuf,
}

impl AssetSourcePlugin {
    /// The default source id.
    pub const DEFAULT_ID: &'static str = "frog";

    /// Returns the default path for the asset source.
    ///
    /// See [`dirs::config_dir`] for more information.
    ///
    /// # Panics
    /// Panics if the config directory cannot be found.
    #[must_use]
    pub fn default_path() -> PathBuf {
        dirs::config_dir().expect("Failed to find config directory").join("FrogLight")
    }
}

impl Default for AssetSourcePlugin {
    fn default() -> Self {
        Self { source_id: Self::DEFAULT_ID.to_string(), source_path: Self::default_path() }
    }
}

impl Plugin for AssetSourcePlugin {
    fn build(&self, app: &mut App) {
        if let Some(path) = self.source_path.to_str() {
            if !self.source_path.exists() {
                warn!("Creating \"{}\"...", self.source_path.display());
                if let Err(err) = std::fs::create_dir(&self.source_path) {
                    error!("Failed to create path \"{}\": {err}", self.source_path.display());
                }
            }

            app.register_asset_source(
                self.source_id.clone(),
                AssetSourceBuilder::platform_default(path, None),
            );
        } else {
            error!(
                "Failed to read path \"{}\" for asset source \"{}\"",
                self.source_path.display(),
                self.source_id
            );
        }
    }
}
