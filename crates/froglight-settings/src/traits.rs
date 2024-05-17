use std::path::Path;

use bevy_app::{App, AppExit, PostUpdate, PreStartup};
use bevy_ecs::{
    schedule::{
        common_conditions::{not, on_event, resource_added, resource_exists_and_changed},
        Condition, IntoSystemConfigs,
    },
    system::{Commands, Res, Resource},
};
use bevy_log::{debug, error};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    systemsets::{SettingsPostUpdateSet, SettingsPreStartupSet},
    ConfigFolder,
};

/// A trait implementing basic loading and saving for configuration files.
pub trait ConfigFile: Default + Clone + Resource + Serialize + DeserializeOwned {
    /// The path to the configuration file, relative to the [`ConfigFolder`]
    const PATH: &'static str;

    /// Builds the configuration systems.
    fn build(app: &mut App) {
        // Add systems for loading and saving the file
        app.add_systems(PreStartup, load_file::<Self>.in_set(SettingsPreStartupSet));
        app.add_systems(
            PostUpdate,
            save_file::<Self>
                .run_if(resource_exists_and_changed::<Self>.or_else(on_event::<AppExit>()))
                .run_if(not(resource_added::<Self>))
                .in_set(SettingsPostUpdateSet),
        );
    }

    /// Deserializes the configuration file.
    ///
    /// If the file either does not exist or is invalid,
    /// the default configuration is returned.
    fn deserialize(path: &Path) -> Self {
        let path = path.join(Self::PATH);

        // Read the file
        let Ok(contents) = std::fs::read_to_string(&path) else {
            error!("Failed to read file: \"{}\"", path.display());
            return Self::default();
        };

        // Parse the file
        match toml::from_str(&contents) {
            Ok(config) => Self::deserialize_map(config),
            #[allow(unused_variables)]
            Err(err) => {
                #[cfg(debug_assertions)]
                error!("Failed to parse configuration file at \"{}\": {err}", path.display());
                #[cfg(not(debug_assertions))]
                error!("Failed to parse configuration file at \"{}\"", path.display());
                Self::default()
            }
        }
    }

    /// Perform operations after deserializing the file.
    ///
    /// By default, this does nothing.
    #[must_use]
    fn deserialize_map(self) -> Self { self }

    /// Serializes the configuration file.
    fn serialize(self, path: &Path) {
        let path = path.join(Self::PATH);

        let Ok(contents) = toml::to_string_pretty(&Self::serialize_map(self)) else {
            error!("Failed to serialize configuration file at \"{}\"", path.display());
            return;
        };

        #[allow(unused_variables)]
        if let Err(err) = std::fs::write(&path, contents) {
            #[cfg(debug_assertions)]
            error!("Failed to write configuration file at \"{}\": {err}", path.display());
            #[cfg(not(debug_assertions))]
            error!("Failed to write configuration file at \"{}\"", path.display());
        }
    }

    /// Perform operations before serializing the file.
    ///
    /// By default, this does nothing.
    #[must_use]
    fn serialize_map(self) -> Self { self }
}

/// Loads the configuration file.
fn load_file<T: ConfigFile>(folder: Res<ConfigFolder>, mut commands: Commands) {
    debug!("Loading `{}` from \"{}\"", std::any::type_name::<T>(), T::PATH);
    commands.insert_resource(<T as ConfigFile>::deserialize(&folder.path));
}

/// Saves the configuration file.
fn save_file<T: ConfigFile + Clone>(folder: Res<ConfigFolder>, config: Res<T>) {
    debug!("Saving `{}` to \"{}\"", std::any::type_name::<T>(), T::PATH);
    <T as ConfigFile>::serialize(config.clone(), &folder.path);
}
