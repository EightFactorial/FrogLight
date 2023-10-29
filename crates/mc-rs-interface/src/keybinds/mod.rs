use std::{fs, path::PathBuf};

use bevy::{app::AppExit, prelude::*};
use serde::{Deserialize, Serialize};

pub mod bundle;

pub mod gameplay;
use gameplay::GameplayKeybinds;

pub mod inventory;
use inventory::InventoryKeybinds;

pub mod movement;
use movement::MovementKeybinds;

use crate::util::dir::config_folder;

pub(super) fn setup(app: &mut App) {
    // Load keybinds from file
    app.insert_resource(KeyBinds::load());

    // Add systems to save settings
    app.add_systems(
        Update,
        KeyBinds::save_keybinds
            .run_if(resource_exists_and_changed::<KeyBinds>().or_else(KeyBinds::exit_event)),
    );

    // Setup submodules
    gameplay::setup(app);
    inventory::setup(app);
    movement::setup(app);

    bundle::setup(app);
}

/// A set of keybinds for the player.
///
/// This is a bundle of all keybinds for the player, separated into categories.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Resource)]
pub struct KeyBinds {
    #[serde(default)]
    pub movement: MovementKeybinds,
    #[serde(default)]
    pub gameplay: GameplayKeybinds,
    #[serde(default)]
    pub inventory: InventoryKeybinds,
}

impl KeyBinds {
    /// Get the default path for the keybinds file.
    ///
    /// TODO: Find proper location for keybinds
    fn default_path() -> PathBuf { config_folder().join("keybinds.toml") }

    /// Load keybinds from the `keybinds.toml` file.
    pub fn load() -> Self {
        #[cfg(any(debug_assertions, feature = "debug"))]
        {
            debug!("Loading keybinds from `{}`", Self::default_path().display());
        }

        // Try to read the file
        match fs::read_to_string(Self::default_path()) {
            // File does not exist, return default keybinds
            Err(err) => {
                error!("Could not read keybinds file: {err}");

                KeyBinds::default()
            }
            // File exists, try to parse it
            Ok(string) => match toml::from_str::<KeyBinds>(&string) {
                // File is invalid, return default keybinds
                Err(err) => {
                    error!("Could not parse keybinds file: {err}");

                    KeyBinds::default()
                }
                // Return parsed keybinds
                Ok(keybinds) => keybinds,
            },
        }
    }

    /// Save keybinds to the `keybinds.toml` file.
    pub fn save(&self) {
        // Try to serialize the keybinds
        match toml::to_string_pretty(self) {
            // Settings are invalid, print error
            Err(err) => error!("Could not serialize keybinds: {err}"),
            // Try to write the keybinds to the file
            Ok(string) => {
                if let Err(err) = fs::write(Self::default_path(), string) {
                    // File could not be written, print error
                    error!("Could not write keybinds file: {err}");
                }
            }
        }
    }

    /// A condition that checks for the app exit event.
    fn exit_event(events: EventReader<AppExit>) -> bool {
        if !events.is_empty() {
            #[cfg(any(debug_assertions, feature = "debug"))]
            {
                debug!("App exit event received, saving keybinds");
            }

            true
        } else {
            false
        }
    }

    /// A system that saves the keybinds.
    fn save_keybinds(keybinds: Res<KeyBinds>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        {
            debug!(
                "Saving keybinds to `{}`",
                KeyBinds::default_path().display()
            );
        }

        keybinds.save();
    }
}
