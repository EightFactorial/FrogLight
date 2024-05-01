#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use std::path::PathBuf;

use bevy_app::{App, Plugin};

mod folder;
pub use folder::ConfigFolder;

pub mod systemsets;

mod traits;
pub use traits::ConfigFile;

/// The `Settings` Froglight plugin.
///
/// Adds a configuration folder to the application,
/// and optionally registers an asset source for it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SettingsPlugin {
    /// The path to the configuration folder.
    ///
    /// If `None`, the [`default path`](ConfigFolder::from_env) is used.
    pub path: Option<PathBuf>,

    /// Whether to add an `AssetSource` for the configuration folder.
    ///
    /// If `true`, paths using `frog://` can be used to access
    /// the configuration folder.
    ///
    /// Does nothing is the `bevy_asset` feature is disabled.
    pub asset_source: bool,
}

impl Default for SettingsPlugin {
    fn default() -> Self { Self { path: None, asset_source: true } }
}

impl SettingsPlugin {
    /// Creates a new [`SettingsPlugin`] with the given path.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: Some(path.into()), asset_source: true }
    }

    /// Disables registering an asset source for the configuration folder.
    #[must_use]
    pub fn disable_source(self) -> Self { Self { asset_source: false, ..self } }
}

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        systemsets::build(app);
        folder::build(self, app);
    }
}
