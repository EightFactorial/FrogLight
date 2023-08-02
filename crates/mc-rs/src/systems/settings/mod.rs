use std::{fs, path::PathBuf};

use bevy::prelude::Resource;
use log::{error, info, log_enabled, Level};
use serde::{Deserialize, Serialize};

pub mod window;
use window::WindowSettings;

pub mod menu;
use menu::MenuSettings;

/// Settings for the application.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Resource)]
pub struct Settings {
    #[serde(default)]
    pub window: WindowSettings,
    #[serde(default)]
    pub menu: MenuSettings,
}

impl Settings {
    /// Load settings from file.
    pub fn load() -> Self {
        match fs::read_to_string(Self::get_path()) {
            Ok(string) => toml::from_str(&string).unwrap_or_else(|err| {
                Self::log_err(format!("Error parsing settings: {err}"));

                Self::default().save_and_return()
            }),
            Err(err) => {
                if matches!(err.kind(), std::io::ErrorKind::NotFound) {
                    Self::log("Creating default settings file...".to_string());
                } else {
                    Self::log_err(format!("Error loading settings: {err}"));
                }

                Self::default().save_and_return()
            }
        }
    }

    /// Save settings to file.
    pub fn save(&self) {
        if let Ok(data) = toml::to_string(self) {
            if let Err(e) = fs::write(Self::get_path(), data) {
                Self::log_err(format!("Error saving settings: {e}"));
            }
        }
    }

    /// Shortcut for saving and returning self.
    fn save_and_return(self) -> Self {
        self.save();
        self
    }

    // TODO: Get path based on OS
    // Something like,
    //
    // Windows: C:\Users\<user>\AppData\Roaming\mc-rs\settings.toml
    // Linux: ~/.config/mc-rs/settings.toml
    // Mac (?): ~/Library/Application Support/mc-rs/settings.toml

    /// Get the path to the settings file.
    fn get_path() -> PathBuf { "settings.toml".into() }

    /// Log a message to the console.
    fn log(msg: String) {
        if log_enabled!(Level::Info) {
            info!("{msg}");
        } else {
            println!("{msg}");
        }
    }
    /// Log an error to the console.
    fn log_err(err: String) {
        if log_enabled!(Level::Error) {
            error!("{err}");
        } else {
            eprintln!("{err}");
        }
    }
}
