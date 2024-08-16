use std::path::PathBuf;

use bevy_app::{App, Plugin};
use bevy_asset::{io::AssetSourceBuilder, AssetApp};
use bevy_log::{error, warn};

/// A [`Plugin`] that registers an asset source.
///
/// Allows assets to be loaded from the provided folder
/// using `source_id://path/to/asset`.
///
/// Example:
/// ```rust,ignore
/// // Use the default `AssetSourcePlugin`.
/// app.add_plugins(AssetSourcePlugin::default());
///
/// // Later, in a system:
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
/// struct ResourcePackHandle(Handle<ResourcePack>);
///
/// fn load_resourcepack(asset_server: Res<AssetServer>, mut commands: Commands) {
///     // Load a `ResourcePack` from `Froglight/resourcepack/my_resourcepack.zip`.
///     let handle: Handle<ResourcePack> = asset_server.load("frog://resourcepack/my_resourcepack.zip");
///     // Store the handle so it doesn't get dropped.
///     commands.insert_resource(ResourcePackHandle(handle));
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetSourcePlugin {
    /// The source id.
    pub source_id: String,
    /// The folder path.
    pub folder_path: PathBuf,
}

impl Default for AssetSourcePlugin {
    fn default() -> Self { Self::new() }
}

impl AssetSourcePlugin {
    /// The default source id.
    pub const DEFAULT_SOURCE: &'static str = "frog";
    /// The default folder name.
    pub const DEFAULT_FOLDER: &'static str = "FrogLight";

    /// Creates a new [`AssetSourcePlugin`] with the default source id and path.
    ///
    /// By default, assets can be loaded from the `Froglight` folder in the
    /// user's config directory using `frog://path/to/asset`.
    ///
    /// See [`dirs::config_dir`] for more information.
    ///
    /// # Panics
    /// Panics if the config directory cannot be found.
    #[must_use]
    pub fn new() -> Self {
        let config_folder = dirs::config_dir().expect("Failed to find config directory");
        let folder_path = config_folder.join(Self::DEFAULT_FOLDER);

        Self { source_id: String::from(Self::DEFAULT_SOURCE), folder_path }
    }
}

impl Plugin for AssetSourcePlugin {
    /// Allow multiple [`AssetSourcePlugin`]s to be registered.
    fn is_unique(&self) -> bool { false }

    fn build(&self, app: &mut App) {
        // Get the folder path.
        let Some(path) = self.folder_path.to_str() else {
            error!(
                "Failed to read path \"{}\" for asset source \"{}\"",
                self.folder_path.display(),
                self.source_id
            );
            return;
        };

        // Create the folder if it doesn't exist.
        if !self.folder_path.exists() {
            warn!("Creating \"{}\"...", self.folder_path.display());
            if let Err(err) = std::fs::create_dir(&self.folder_path) {
                error!("Failed to create path \"{}\": {err}", self.folder_path.display());
            }
        }

        #[cfg(debug_assertions)]
        bevy_log::info!("AssetSource: Registering \"{}\" -> \"{}\"", self.source_id, path);

        // Register the asset source.
        app.register_asset_source(
            self.source_id.clone(),
            AssetSourceBuilder::platform_default(path, None),
        );
    }
}
