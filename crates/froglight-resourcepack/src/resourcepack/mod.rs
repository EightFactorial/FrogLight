use bevy::{prelude::*, utils::HashMap};

pub mod meta;
use meta::PackMcMeta;

/// A resource pack.
///
/// A collection of many kinds of assets and metadata.
#[derive(Debug, Default, Clone, PartialEq, Eq, Asset, Reflect)]
pub struct ResourcePack {
    /// The resource pack's metadata
    pub meta: PackMcMeta,
    /// The resource pack's icon
    pub icon: Option<Handle<Image>>,

    /// Weak handles to the resource pack's textures
    pub textures: HashMap<String, Handle<Image>>,
}
