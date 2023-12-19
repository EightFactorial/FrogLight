use bevy::{prelude::*, utils::HashMap};
use compact_str::CompactString;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, Deserialize)]
pub struct ResourcePackMetaContainer {
    pub pack: Option<ResourcePackMeta>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ResourcePackMeta {
    #[serde(rename = "pack_format")]
    pub version: u32,
    pub description: CompactString,

    #[serde(flatten)]
    pub extra: HashMap<CompactString, serde_json::Value>,
}

impl From<ResourcePackMeta> for ResourcePackMetaContainer {
    fn from(pack: ResourcePackMeta) -> Self { Self { pack: Some(pack) } }
}

impl From<Option<ResourcePackMeta>> for ResourcePackMetaContainer {
    fn from(pack: Option<ResourcePackMeta>) -> Self { Self { pack } }
}
