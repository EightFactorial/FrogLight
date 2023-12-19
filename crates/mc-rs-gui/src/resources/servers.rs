use bevy::prelude::*;
use compact_str::CompactString;

#[derive(Debug, Default, Clone, PartialEq, Eq, Resource, Deref, DerefMut)]
pub struct ServerList {
    pub servers: Vec<ServerItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServerItem {
    pub title: CompactString,
    pub address: CompactString,

    pub cached_status: Option<CompactString>,
    pub cached_icon: Option<CompactString>,
}
