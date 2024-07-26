//! Unprocessed assets are assets that are read from disk.
//!
//! These may need to be processed before they can be used.

use bevy_app::App;
use bevy_asset::{AssetApp, Handle, ReflectHandle};

pub(crate) mod atlas_definition;
pub use atlas_definition::ResourceAtlasDefinition;

pub(crate) mod block_definition;
pub use block_definition::BlockModelDefinition;

pub(crate) mod language_map;
pub use language_map::LanguageMap;

pub(crate) mod sound_definition;
pub use sound_definition::SoundDefinitionMap;

pub(crate) mod resourcepack;
pub use resourcepack::ResourcePack;

pub(crate) mod resourcepack_meta;
pub use resourcepack_meta::ResourcePackMeta;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Register `ResourceAtlasDefinition`
    app.register_type::<ResourceAtlasDefinition>()
        .register_type::<Handle<ResourceAtlasDefinition>>()
        .register_type_data::<Handle<ResourceAtlasDefinition>, ReflectHandle>()
        .init_asset::<ResourceAtlasDefinition>();

    // Register `BlockDefinition`
    app.register_type::<BlockModelDefinition>()
        .register_type::<Handle<BlockModelDefinition>>()
        .register_type_data::<Handle<BlockModelDefinition>, ReflectHandle>()
        .init_asset::<BlockModelDefinition>();

    // Register `LanguageMap`
    app.register_type::<LanguageMap>()
        .register_type::<Handle<LanguageMap>>()
        .register_type_data::<Handle<LanguageMap>, ReflectHandle>()
        .init_asset::<LanguageMap>();

    // Register `SoundDefinitionMap`
    app.register_type::<SoundDefinitionMap>()
        .register_type::<Handle<SoundDefinitionMap>>()
        .register_type_data::<Handle<SoundDefinitionMap>, ReflectHandle>()
        .init_asset::<SoundDefinitionMap>();

    // Register `ResourcePackMeta`
    app.register_type::<ResourcePackMeta>()
        .register_type::<Handle<ResourcePackMeta>>()
        .register_type_data::<Handle<ResourcePackMeta>, ReflectHandle>()
        .init_asset::<ResourcePackMeta>();

    // Register `ResourcePack`
    app.register_type::<ResourcePack>()
        .register_type::<Handle<ResourcePack>>()
        .register_type_data::<Handle<ResourcePack>, ReflectHandle>()
        .init_asset::<ResourcePack>();
}
