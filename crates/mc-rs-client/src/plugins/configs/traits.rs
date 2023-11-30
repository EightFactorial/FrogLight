use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use bevy::{app::AppExit, prelude::*};
use serde::{de::DeserializeOwned, Serialize};

use crate::dir::config_folder;

/// A trait that adds bevy systems to update a config file resource.
pub(crate) trait ResourceConfig: ConfigFile + Resource {
    /// Adds systems to the app to update the config file.
    fn add_systems(app: &mut App) {
        app.add_systems(Update, Self::save_config.run_if(on_event::<AppExit>()));
    }

    /// A bevy system that saves the config file.
    fn save_config(config: Res<Self>) {
        if let Err(err) = config.save() {
            error!("Failed to save config file: {err}");
        }
    }
}

/// A trait that adds methods to load and save a config file.
pub(crate) trait ConfigFile: Default + Serialize + DeserializeOwned {
    /// The path to the config file relative to the config folder.
    const FILE_PATH: &'static str;
    /// The path to the config file
    fn get_path() -> PathBuf { config_folder().join(Self::FILE_PATH) }

    /// Load the config file from disk or use the default.
    fn load() -> Self {
        match Self::try_load() {
            Ok(config) => config,
            Err(err) => {
                error!("Failed to load config file: {err}");
                Self::default()
            }
        }
    }

    /// Attempt to load the config file from disk
    fn try_load() -> anyhow::Result<Self> {
        let path = Self::get_path();
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Loading config file from \"{}\"", path.display());

        if !path.exists() {
            #[cfg(any(debug_assertions, feature = "debug"))]
            warn!(
                "{} file not found at \"{}\", using defaults!",
                stringify!(Self),
                path.display()
            );

            return Ok(Self::default());
        }

        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(toml::from_str(&contents)?)
    }

    /// Attempt to save the config file to disk.
    fn save(&self) -> anyhow::Result<()> {
        let serialized = toml::to_string_pretty(self)?;

        let path = Self::get_path();
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Saving config file to \"{}\"", path.display());

        let mut file = fs::File::create(path)?;
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }
}
