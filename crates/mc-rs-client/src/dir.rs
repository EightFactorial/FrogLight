use std::{fs, path::PathBuf};

pub(crate) fn config_folder() -> PathBuf {
    // If `~/.MC-RS` (Linux) or `C:\Users\<user>\.MC-RS` (Windows) exists, use that
    let home_dir = dirs::home_dir().map(|path| path.join(".MC-RS"));
    if let Some(true) = home_dir.as_ref().map(|path| path.exists()) {
        return home_dir.unwrap();
    }

    // Otherise, use the config directory `~/.config/MC-RS` (Linux) or `%APPDATA%/MC-RS` (Windows)
    let Some(config_dir) = dirs::config_dir().map(|path| path.join("MC-RS")) else {
        panic!("Could not find config directory");
    };

    // Create the config directory if it does not exist
    if !config_dir.exists() {
        #[cfg(any(debug_assertions, feature = "debug"))]
        bevy::prelude::debug!("Creating config directory at `{}`", config_dir.display());

        if let Err(err) = fs::create_dir_all(&config_dir) {
            panic!("Could not create config directory: {}", err);
        }
    }

    config_dir
}
