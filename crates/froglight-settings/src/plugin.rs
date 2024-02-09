use std::path::{Path, PathBuf};

use bevy::prelude::*;
use compact_str::CompactString;

/// The [`Plugin`] for the [`froglight-settings`](crate) crate.
///
/// Adds support for loading files from the config directory by adding the
/// `frog://` asset source.
///
/// # Example
/// ```no_run,ignore
/// use bevy::prelude::*;
///
/// // Load an image from the config directory
/// let image = asset_server.load("frog://image.png");
/// ```
///
/// ---
///
/// Default directory varies by platform:
///
/// |Platform | Value                                                     | Example                                              |
/// | ------- | --------------------------------------------------------- | ---------------------------------------------------- |
/// | Linux   | `$XDG_CONFIG_HOME/FrogLight` or `$HOME/.config/FrogLight` | `/home/alice/.config/FrogLight`                      |
/// | macOS   | `$HOME/Library/Application Support/FrogLight`             | `/Users/Alice/Library/Application Support/FrogLight` |
/// | Windows | `{FOLDERID_RoamingAppData}\FrogLight`                     | `C:\Users\Alice\AppData\Roaming\FrogLight`           |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsPlugin {
    dir_path: PathBuf,
    dir_name: CompactString,
}

impl SettingsPlugin {
    /// The default directory name for the config directory.
    pub const DEFAULT_DIR: CompactString = CompactString::new_inline("FrogLight");

    /// Creates a new [`SettingsPlugin`] with the given config directory.
    ///
    /// If the provided `dir_path` is `None`, the directory provided by
    /// [`SettingsPlugin::default_directory`] will be used.
    ///
    /// # Panics
    /// - If the `dir_path` is a file.
    /// - If the directory cannot be created.
    #[must_use]
    pub fn new(dir_path: Option<impl Into<PathBuf>>, dir_name: impl Into<CompactString>) -> Self {
        let mut dir_path = match dir_path {
            Some(path) => path.into(),
            None => Self::default_directory(),
        };

        if dir_path.is_file() {
            // If the directory path is a file, use the default directory
            error!("The config directory path is a file: `{}`", dir_path.display());
            dir_path = Self::default_directory();
        } else if dir_path.as_os_str().is_empty() {
            // If the directory path is empty, use the default directory
            error!("The provided config directory path is empty");
            dir_path = Self::default_directory();
        }

        Self { dir_path, dir_name: dir_name.into() }
    }

    /// Gets the full path for the config directory based on the `dir_path` and
    /// `dir_name` provided in [`SettingsPlugin::new`].
    ///
    /// # Example
    /// ```no_run
    /// use froglight_settings::SettingsPlugin;
    ///
    /// let plugin = SettingsPlugin::default();
    /// let dir = plugin.get_directory();
    ///
    /// // On Linux, the directory might be:
    /// #[cfg(target_os = "linux")]
    /// assert_eq!(dir.to_str().unwrap(), "/home/alice/.config/FrogLight");
    ///
    /// // On Windows, the directory might be:
    /// #[cfg(target_os = "windows")]
    /// assert_eq!(dir.to_str().unwrap(), "C:\\Users\\Alice\\AppData\\Roaming\\FrogLight");
    /// ```
    #[must_use]
    pub fn get_directory(&self) -> PathBuf { self.dir_path.join(self.dir_name.as_str()) }

    /// The default directory for the config directory.
    ///
    /// Used when the `dir_path` is `None` in [`SettingsPlugin::new`].
    ///
    /// # Panics
    /// - If the environment variable(s) to get the config directory are not
    ///   set.
    #[must_use]
    pub fn default_directory() -> PathBuf {
        dirs::config_dir().expect("Unable to get config directory")
    }
}

impl Default for SettingsPlugin {
    fn default() -> Self { Self::new(Some(Self::default_directory()), Self::DEFAULT_DIR) }
}

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        // Create the directory if it doesn't exist
        if !self.dir_path.exists() {
            info!("Creating the config directory: `{}`", self.dir_path.display());

            if let Err(err) = std::fs::create_dir_all(&self.dir_path) {
                panic!(
                    "Failed to create the config directory `{}`: `{err}`",
                    self.dir_path.display()
                );
            }
        }

        // Add the SettingsSource to the app
        app.insert_resource(SettingsSource(self.dir_path.clone()));

        // Register the asset source
        crate::source::build(app);
    }
}

/// The path to the settings directory.
///
/// This is not changed after the program starts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct SettingsSource(pub(crate) PathBuf);

impl SettingsSource {
    /// Gets the current config directory.
    #[must_use]
    pub fn path(&self) -> &Path { &self.0 }
}
