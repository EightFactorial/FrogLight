use bevy_app::{App, AppExit, PostUpdate, PreStartup};
use bevy_ecs::{
    schedule::{
        common_conditions::{
            not, on_event, resource_added, resource_changed, resource_exists, run_once,
        },
        IntoSystemConfigs,
    },
    system::{Commands, Res, Resource},
};
use bevy_log::{debug, error};
use bevy_reflect::GetTypeRegistration;
use froglight_core::systemsets::{AssetPostUpdateSet, AssetPreStartupSet};
use serde::{de::DeserializeOwned, Serialize};

use crate::AssetSource;

pub(crate) mod plugin;

pub(crate) mod resourcepack_config;
use resourcepack_config::ResourcePackSettings;

/// Add config file systems to the given app.
#[doc(hidden)]
pub(crate) fn build(app: &mut App) { ResourcePackSettings::register(app); }

/// A trait implemented for config files
///
/// Allows for easy loading and saving of config files.
trait ConfigFile: Default + Resource + GetTypeRegistration + Serialize + DeserializeOwned {
    const FILEPATH: &'static str;

    /// Register the config file with the given app.
    fn register(app: &mut App) {
        // Register the type
        app.register_type::<Self>();

        // Load the file during startup
        app.add_systems(
            PreStartup,
            load_config_on_startup::<Self>
                .run_if(not(resource_exists::<Self>))
                .run_if(run_once())
                .in_set(AssetPreStartupSet),
        );

        // Save the file when modified
        app.add_systems(
            PostUpdate,
            save_config_on_modify::<Self>
                .run_if(resource_changed::<Self>)
                .run_if(not(resource_added::<Self>))
                .in_set(AssetPostUpdateSet),
        );

        // Save the file when exiting
        app.add_systems(
            PostUpdate,
            save_config_on_exit::<Self>
                .run_if(resource_exists::<Self>)
                .run_if(on_event::<AppExit>())
                .in_set(AssetPostUpdateSet),
        );

        // Add any custom systems
        Self::build(app);
    }

    /// Add custom systems to the given app.
    fn build(_app: &mut App) {}

    /// Load the config file from the given source.
    fn load(source: &AssetSource) -> Self {
        let path = source.join(Self::FILEPATH);

        match std::fs::read_to_string(&path) {
            Err(err) => {
                error!("Failed to read: `{}`: {err}", path.display());
                Self::default()
            }
            Ok(file) => match toml::from_str(&file) {
                Err(err) => {
                    error!("Failed to parse: `{}`: {err}", path.display());
                    Self::default()
                }
                Ok(config) => {
                    debug!("Loaded: `{}`", path.display());
                    config
                }
            },
        }
    }

    /// Save the config file inside the given source.
    fn save(config: &Self, source: &AssetSource) {
        let path = source.join(Self::FILEPATH);

        match toml::to_string_pretty(config) {
            Err(err) => error!("Failed to serialize: `{}`: {err}", path.display()),
            Ok(file) => {
                if let Err(err) = std::fs::write(&path, file) {
                    error!("Failed to write: `{}`: {err}", path.display());
                } else {
                    debug!("Saved: `{}`", path.display());
                }
            }
        }
    }
}

fn load_config_on_startup<T: ConfigFile>(source: Res<AssetSource>, mut commands: Commands) {
    commands.insert_resource(T::load(&source));
}

fn save_config_on_modify<T: ConfigFile>(config: Res<T>, source: Res<AssetSource>) {
    T::save(&*config, &source);
}

fn save_config_on_exit<T: ConfigFile>(config: Res<T>, source: Res<AssetSource>) {
    T::save(&*config, &source);
}
