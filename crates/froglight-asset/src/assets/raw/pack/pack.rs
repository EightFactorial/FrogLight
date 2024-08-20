use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_audio::AudioSource;
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::texture::Image;
use bevy_utils::HashMap;
use froglight_common::ResourceKey;

use crate::assets::{
    raw::{SingleLanguageMap, SoundDefinitionMap},
    ResourcePackMeta,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset::<ResourcePack>();
    app.register_type::<ResourcePack>()
        .register_type::<Handle<ResourcePack>>()
        .register_type_data::<Handle<ResourcePack>, ReflectHandle>();
}

/// A resource pack.
#[derive(Debug, Default, Clone, Asset, Reflect)]
#[reflect(Default, Asset)]
pub struct ResourcePack {
    /// The [`ResourcePack`]'s metadata.
    pub meta: Handle<ResourcePackMeta>,

    /// The [`ResourcePack`]'s textures.
    pub textures: HashMap<ResourceKey, Handle<Image>>,

    /// The [`ResourcePack`]'s sounds.
    pub sounds: HashMap<ResourceKey, Handle<AudioSource>>,
    /// The [`ResourcePack`]'s sound maps.
    pub sound_maps: HashMap<ResourceKey, Handle<SoundDefinitionMap>>,

    /// The [`ResourcePack`]'s languages.
    pub languages: HashMap<ResourceKey, Handle<SingleLanguageMap>>,

    /// Other [`ResourcePack`]s embedded in this [`ResourcePack`].
    pub children: HashMap<ResourceKey, Handle<ResourcePack>>,
}

impl ResourcePack {
    /// Creates a new [`ResourcePack`] with the given metadata.
    #[must_use]
    pub fn new(meta: Handle<ResourcePackMeta>) -> Self { Self { meta, ..Self::default() } }
}
