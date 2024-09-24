use bevy_app::App;
use bevy_asset::{
    Asset, AssetApp, Handle, ReflectAsset, ReflectHandle, UntypedAssetId, VisitAssetDependencies,
};
use bevy_audio::AudioSource;
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::texture::Image;
use bevy_utils::HashMap;
use froglight_common::ResourceKey;

use crate::assets::{
    raw::{BlockModelDefinition, BlockStateDefinition, SingleLanguageMap, SoundDefinitionMap},
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
#[derive(Debug, Default, Clone, Reflect)]
#[reflect(Default, Asset)]
pub struct ResourcePack {
    /// The [`ResourcePack`]'s metadata.
    pub meta: Handle<ResourcePackMeta>,

    /// The [`ResourcePack`]'s textures.
    pub textures: HashMap<ResourceKey, Handle<Image>>,

    /// The [`ResourcePack`]'s block models.
    pub block_models: HashMap<ResourceKey, Handle<BlockModelDefinition>>,
    /// The [`ResourcePack`]'s block states.
    pub block_states: HashMap<ResourceKey, Handle<BlockStateDefinition>>,

    /// The [`ResourcePack`]'s sounds.
    pub sounds: HashMap<ResourceKey, Handle<AudioSource>>,
    /// The [`ResourcePack`]'s sound maps.
    pub sound_maps: HashMap<ResourceKey, Handle<SoundDefinitionMap>>,

    /// The [`ResourcePack`]'s languages.
    pub languages: HashMap<ResourceKey, Handle<SingleLanguageMap>>,

    /// Other [`ResourcePack`]s embedded in this [`ResourcePack`].
    pub children: HashMap<ResourceKey, Handle<ResourcePack>>,
}

impl Asset for ResourcePack {}
impl VisitAssetDependencies for ResourcePack {
    fn visit_dependencies(&self, visit: &mut impl FnMut(UntypedAssetId)) {
        self.meta.visit_dependencies(visit);
        self.textures.values().for_each(visit_handle(visit));
        self.block_models.values().for_each(visit_handle(visit));
        self.block_states.values().for_each(visit_handle(visit));
        self.sounds.values().for_each(visit_handle(visit));
        self.sound_maps.values().for_each(visit_handle(visit));
        self.languages.values().for_each(visit_handle(visit));
        self.children.values().for_each(visit_handle(visit));
    }
}

#[inline]
fn visit_handle<A: Asset>(visit: &mut impl FnMut(UntypedAssetId)) -> impl '_ + FnMut(&Handle<A>) {
    |handle| handle.visit_dependencies(visit)
}

impl ResourcePack {
    /// Creates a new [`ResourcePack`] with the given metadata.
    #[must_use]
    pub fn new(meta: Handle<ResourcePackMeta>) -> Self { Self { meta, ..Self::default() } }
}
