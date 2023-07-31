#![allow(dead_code)]

use std::{fs, path::PathBuf};

use bevy::prelude::Resource;
use log::error;
use serde::{Deserialize, Serialize};

pub mod window;
use window::WindowSettings;

/// Settings for the application.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Resource)]
pub struct Settings {
    pub window: WindowSettings,
}

impl Settings {
    /// Load settings from file.
    pub fn load() -> Self {
        match fs::read_to_string(Self::get_path()) {
            Ok(string) => toml::from_str(&string).unwrap_or_else(|err| {
                eprintln!("Failed to parse settings: {err}");
                Self::default()
            }),
            Err(err) => {
                eprintln!("Error opening settings: {err}");
                Self::default()
            }
        }
    }

    /// Save settings to file.
    pub fn save(&self) {
        if let Ok(data) = toml::to_string(self) {
            if let Err(e) = fs::write(Self::get_path(), data) {
                error!("Failed to save settings: {}", e);
            }
        }
    }

    // TODO: Get path based on OS
    // Something like,
    //
    // Windows: C:\Users\<user>\AppData\Roaming\mc-rs\settings.toml
    // Linux: ~/.config/mc-rs/settings.toml
    // Mac (?): ~/Library/Application Support/mc-rs/settings.toml
    /// Get the path to the settings file.
    fn get_path() -> PathBuf { "settings.toml".into() }
}
