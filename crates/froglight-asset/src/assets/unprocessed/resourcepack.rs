use bevy_asset::{Asset, Handle, ReflectAsset};
use bevy_audio::AudioSource;
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::texture::Image;
use bevy_utils::hashbrown::HashMap;
use froglight_common::ResourceKey;

use super::{
    BlockModelDefinition, LanguageMap, ResourceAtlasDefinition, ResourcePackMeta,
    SoundDefinitionMap,
};

/// A resource pack.
///
/// Contains assets that are read from disk.
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Asset)]
#[reflect(Default, Asset)]
pub struct ResourcePack {
    /// The [`ResourcePack`]'s metadata.
    pub meta: Handle<ResourcePackMeta>,
    /// Other [`ResourcePack`]s embedded in this [`ResourcePack`].
    pub children: HashMap<ResourceKey, Handle<ResourcePack>>,

    /// The [`ResourcePack`]'s textures.
    pub textures: HashMap<ResourceKey, Handle<Image>>,
    /// The [`ResourcePack`]'s sounds.
    pub sounds: HashMap<ResourceKey, Handle<AudioSource>>,
    /// The [`ResourcePack`]'s languages.
    pub languages: HashMap<ResourceKey, Handle<LanguageMap>>,

    /// The [`ResourcePack`]'s block models.
    pub block_models: HashMap<ResourceKey, Handle<BlockModelDefinition>>,

    /// The [`ResourcePack`]'s atlas definitions.
    pub atlas_definitions: HashMap<ResourceKey, Handle<ResourceAtlasDefinition>>,
    /// The [`ResourcePack`]'s sound definitions.
    pub sound_definitions: HashMap<String, Handle<SoundDefinitionMap>>,
}
