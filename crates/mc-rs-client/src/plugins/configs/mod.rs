//! Configuration file plugin

use bevy::prelude::*;

pub mod keybinds;
use keybinds::Keybinds;

pub mod settings;
use settings::Settings;

pub(crate) mod traits;
use traits::{ConfigFile, ResourceConfig};

/// A plugin thats loads all of the config files to the app.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        // Add the keybinds to the app
        Keybinds::add_systems(app);
        app.insert_resource(Keybinds::load());

        // Add the settings to the app
        Settings::add_systems(app);
        let settings = Settings::load();
        settings.insert_resources(app);
        app.insert_resource(settings);
    }
}
