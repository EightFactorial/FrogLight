use std::{fs, path::PathBuf};

use bevy::{app::AppExit, prelude::*};
use serde::{Deserialize, Serialize};

pub mod window;
use window::WindowSettings;

use crate::util::dir::config_folder;

pub(super) fn setup(app: &mut App) -> Settings {
    // Load settings from file
    let settings = Settings::load();
    app.insert_resource(settings.clone());

    #[cfg(any(debug_assertions, feature = "debug"))]
    {
        debug!("Loaded settings:\n{:#?}", settings);
    }

    // Add systems to save settings
    app.add_systems(
        Update,
        Settings::save_settings
            .run_if(resource_exists_and_changed::<Settings>().or_else(Settings::exit_event)),
    );

    // Setup submodules
    window::setup(app);

    settings
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Resource)]
pub struct Settings {
    #[serde(default)]
    pub window: WindowSettings,
}

impl Settings {
    /// Get the default path for the settings file.
    ///
    /// TODO: Find proper location for settings
    fn default_path() -> PathBuf { config_folder().join("settings.toml") }

    /// Load settings from the `settings.toml` file.
    pub fn load() -> Self {
        #[cfg(any(debug_assertions, feature = "debug"))]
        {
            debug!("Loading settings from `{}`", Self::default_path().display());
        }

        // Try to read the file
        match fs::read_to_string(Self::default_path()) {
            // File does not exist, return default settings
            Err(err) => {
                error!("Could not read settings file: {err}");

                Settings::default()
            }
            // File exists, try to parse it
            Ok(string) => match toml::from_str::<Settings>(&string) {
                // File is invalid, return default settings
                Err(err) => {
                    error!("Could not parse settings file: {err}");

                    Settings::default()
                }
                // Return parsed settings
                Ok(settings) => settings,
            },
        }
    }

    /// Save settings to the `settings.toml` file.
    pub fn save(&self) {
        // Try to serialize the settings
        match toml::to_string_pretty(self) {
            // Settings are invalid, print error
            Err(err) => error!("Could not serialize settings: {err}"),
            // Try to write the settings to the file
            Ok(string) => {
                if let Err(err) = fs::write(Self::default_path(), string) {
                    // File could not be written, print error
                    error!("Could not write settings file: {err}");
                }
            }
        }
    }

    /// A condition that checks for the app exit event.
    fn exit_event(events: EventReader<AppExit>) -> bool {
        if !events.is_empty() {
            #[cfg(any(debug_assertions, feature = "debug"))]
            {
                debug!("App exit event received, saving settings");
            }

            true
        } else {
            false
        }
    }

    /// A system that saves the settings.
    fn save_settings(settings: Res<Settings>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        {
            debug!(
                "Saving settings to `{}`",
                Settings::default_path().display()
            );
        }

        settings.save();
    }
}
