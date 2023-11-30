use std::path::PathBuf;
use tracing::{debug, error};

#[cfg(target_os = "linux")]
pub fn minecraft_dir() -> Option<PathBuf> {
    let home = dirs::home_dir()?.join(".minecraft");
    if home.exists() {
        return Some(home);
    }
    debug!("Tried looking in {}", home.display());

    let config = dirs::config_dir()?.join(".minecraft");
    if config.exists() {
        return Some(config);
    }
    debug!("Tried looking in {}", config.display());

    error!("Could not find .minecraft directory!");
    None
}

#[cfg(target_os = "windows")]
pub fn minecraft_dir() -> Option<PathBuf> {
    let appdata = dirs::data_dir()?.join(".minecraft");
    if appdata.exists() {
        return Some(appdata);
    }
    debug!("Tried looking in {}", appdata.display());

    let localappdata = dirs::data_local_dir()?.join(".minecraft");
    if localappdata.exists() {
        return Some(localappdata);
    }
    debug!("Tried looking in {}", localappdata.display());

    let home = dirs::home_dir()?.join(".minecraft");
    if home.exists() {
        return Some(home);
    }
    debug!("Tried looking in {}", home.display());

    error!("Could not find .minecraft directory!");
    None
}

#[cfg(target_os = "macos")]
pub fn minecraft_dir() -> Option<PathBuf> {
    let config = dirs::config_dir()?.join("minecraft");
    if config.exists() {
        return Some(config);
    }
    debug!("Tried looking in {}", config.display());

    let home = dirs::home_dir()?.join(".minecraft");
    if home.exists() {
        return Some(home);
    }
    debug!("Tried looking in {}", home.display());

    error!("Could not find .minecraft directory!");
    None
}
