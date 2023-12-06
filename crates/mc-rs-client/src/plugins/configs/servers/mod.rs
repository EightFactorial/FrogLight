use bevy::{app::AppExit, prelude::*};
use compact_str::CompactString;
use mc_rs_gui::resources::servers::{ServerItem, ServerList};
use serde::{Deserialize, Serialize};

use super::traits::ConfigFile;

#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Serialize, Deserialize)]
pub struct SettingsServerList {
    #[serde(default)]
    pub servers: Vec<SettingsServerItem>,
}

impl ConfigFile for SettingsServerList {
    const FILE_PATH: &'static str = "servers.toml";
}

impl SettingsServerList {
    /// Adds systems to the app to update the config file.
    pub(super) fn add_systems(app: &mut App) {
        app.add_systems(Update, Self::save_config.run_if(on_event::<AppExit>()));
    }

    /// A bevy system that saves the config file.
    pub(super) fn save_config(config: Res<ServerList>) {
        if let Err(err) = Self::from(config.clone()).save() {
            error!("Failed to save config file: {err}");
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SettingsServerItem {
    pub title: CompactString,
    pub address: CompactString,

    pub cached_status: Option<CompactString>,
    pub cached_icon: Option<CompactString>,
}

impl From<SettingsServerList> for ServerList {
    fn from(value: SettingsServerList) -> Self {
        Self {
            servers: value.servers.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ServerList> for SettingsServerList {
    fn from(value: ServerList) -> Self {
        Self {
            servers: value.servers.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<SettingsServerItem> for ServerItem {
    fn from(value: SettingsServerItem) -> Self {
        Self {
            title: value.title,
            address: value.address,
            cached_status: value.cached_status,
            cached_icon: value.cached_icon,
        }
    }
}

impl From<ServerItem> for SettingsServerItem {
    fn from(value: ServerItem) -> Self {
        Self {
            title: value.title,
            address: value.address,
            cached_status: value.cached_status,
            cached_icon: value.cached_icon,
        }
    }
}
