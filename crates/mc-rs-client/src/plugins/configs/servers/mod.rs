use bevy::prelude::*;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use super::traits::{ConfigFile, ResourceConfig};

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Resource, Deref, DerefMut, Serialize, Deserialize,
)]
pub struct ServerList {
    #[serde(default)]
    pub servers: Vec<ServerItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServerItem {
    pub title: CompactString,
    pub address: CompactString,

    #[serde(default)]
    pub cached_status: Option<CompactString>,
    #[serde(default)]
    pub cached_icon: Option<CompactString>,
}

impl ResourceConfig for ServerList {}
impl ConfigFile for ServerList {
    const FILE_PATH: &'static str = "servers.toml";
}
