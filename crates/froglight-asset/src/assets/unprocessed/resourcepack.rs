use bevy_asset::{Asset, Handle, ReflectAsset};
use bevy_audio::AudioSource;
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::texture::Image;
use bevy_utils::HashMap;
use froglight_common::ResourceKey;

use super::{NamespaceSoundMap, ResourcePackMeta};

/// A resource pack.
///
/// Contains assets that are read from disk.
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Asset)]
#[reflect(Default, Asset)]
pub struct ResourcePack {
    /// The [`ResourcePack`]'s metadata.
    pub meta: Handle<ResourcePackMeta>,

    /// The [`ResourcePack`]'s textures.
    pub textures: HashMap<ResourceKey, Handle<Image>>,
    /// The [`ResourcePack`]'s sounds.
    pub sounds: HashMap<ResourceKey, Handle<AudioSource>>,
    /// The [`ResourcePack`]'s soundmaps.
    pub soundmaps: HashMap<String, Handle<NamespaceSoundMap>>,
}
