use bevy::{asset::ReflectAsset, prelude::*, utils::HashMap};
use froglight_core::data::ResourceKey;

pub(crate) mod meta;
use meta::PackMcMeta;

/// A resource pack.
#[derive(Debug, Default, Clone, Asset, Reflect)]
#[reflect(Asset)]
pub struct ResourcePack {
    /// The resource pack's metadata
    pub meta: PackMcMeta,
    /// The resource pack's icon
    pub icon: Option<Handle<Image>>,

    /// Weak handles to the resource pack's audio
    pub audio: HashMap<ResourceKey, Handle<AudioSource>>,
    /// Weak handles to the resource pack's textures
    pub textures: HashMap<ResourceKey, Handle<Image>>,
}
