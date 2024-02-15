use std::io::Read;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::SettingsSource;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Register for reflection
    app.register_type::<ResourcePackSettings>();

    // If the settings are not already loaded, try to load them
    if app.world.get_resource::<ResourcePackSettings>().is_none() {
        // Get the config directory path
        if let Some(path) = app.world.get_resource::<SettingsSource>() {
            // Get the path to the settings file
            let path = path.path().join(ResourcePackSettings::FILE_NAME);

            // Try to open the file
            if let Ok(mut file) = std::fs::File::open(path) {
                // Read the file contents
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    // Try to parse the file contents
                    if let Ok(settings) = toml::from_str(&contents) {
                        trace!("Loaded ResourcePackSettings: {settings:#?}");

                        // Insert the settings into the world
                        app.world.insert_resource::<ResourcePackSettings>(settings);
                    } else {
                        error!("Failed to load ResourcePackSettings: Failed to parse file");
                    }
                } else {
                    error!("Failed to load ResourcePackSettings: Failed to read file");
                }
            } else {
                error!("Failed to load ResourcePackSettings: Failed to open file");
            }
        } else {
            error!("Failed to load ResourcePackSettings: SettingsSource resource not found");
        }
    }

    // If the settings are still not loaded, insert the default settings
    if app.world.get_resource::<ResourcePackSettings>().is_none() {
        warn!("Failed to load ResourcePackSettings, using default settings");

        app.init_resource::<ResourcePackSettings>();
    }
}

/// Settings for all resource packs
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Resource, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct ResourcePackSettings {
    /// List of resource packs
    pub packs: Vec<PackSettings>,
}

/// Settings for a resource pack
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub struct PackSettings {
    /// The path to the resource pack\
    ///
    /// Should be a bevy asset path, with the source being the asset source id.
    ///
    /// Froglight loads resource packs from `frog://resourcepacks`, so
    /// paths should be prefixed with `frog://resourcepacks/`.
    ///
    /// For example, if the resource pack is `my_pack.zip`,
    /// the path should be `frog://resourcepacks/my_pack.zip
    pub path: String,
}

impl ResourcePackSettings {
    /// Get the file name for the [`ResourcePackSettings`] file
    pub const FILE_NAME: &'static str = "resourcepack.toml";
}
