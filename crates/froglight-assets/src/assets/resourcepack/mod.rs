use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_audio::AudioSource;
use bevy_reflect::{std_traits::ReflectDefault, Reflect};
use bevy_render::texture::Image;
use froglight_components::resourcekey::ResourceKey;
use hashbrown::HashMap;

mod info;
pub use info::*;

mod loader;
pub use loader::*;

use super::{
    blockstate::BlockStateDefinition, language::LanguageFile, model::ModelDefinition,
    particle::ParticleDefinition, sound::SoundDefinitions, textsource::TextSource, FontDefinition,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset::<ResourcePack>().init_asset_loader::<ResourcePackZipLoader>();
    app.register_type::<ResourcePack>().register_type_data::<Handle<ResourcePack>, ReflectHandle>();

    info::build(app);
}

/// A bundle of assets.
#[derive(Debug, Default, Clone, PartialEq, Asset, Reflect)]
#[reflect(Default, Asset)]
pub struct ResourcePack {
    /// [`ResourcePack`] information
    pub info: ResourcePackInfo,

    /// Blockstates
    pub blockstates: HashMap<ResourceKey, BlockStateDefinition>,
    /// Fonts
    pub fonts: HashMap<ResourceKey, FontDefinition>,
    /// Languages
    pub lang: HashMap<ResourceKey, LanguageFile>,
    /// Models
    pub models: HashMap<ResourceKey, ModelDefinition>,
    /// Particles
    pub particles: HashMap<ResourceKey, ParticleDefinition>,
    /// Sounds
    pub sounds: HashMap<ResourceKey, Handle<AudioSource>>,
    /// Sound Definitions
    pub sound_defs: HashMap<ResourceKey, SoundDefinitions>,
    /// Texts
    pub texts: HashMap<ResourceKey, TextSource>,
    /// Textures
    pub textures: HashMap<ResourceKey, Handle<Image>>,
}
