//! Unprocessed assets are assets that are read from disk.
//!
//! These may need to be processed before they can be used.

use bevy_app::App;
use bevy_asset::{AssetApp, Handle, ReflectHandle};

mod language_map;
pub use language_map::LanguageMap;

mod namespace_soundmap;
pub use namespace_soundmap::NamespaceSoundMap;

mod resourcepack;
pub use resourcepack::ResourcePack;

mod resourcepack_meta;
pub use resourcepack_meta::ResourcePackMeta;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Register `LanguageMap`
    app.register_type::<LanguageMap>()
        .register_type::<Handle<LanguageMap>>()
        .register_type_data::<Handle<LanguageMap>, ReflectHandle>()
        .init_asset::<LanguageMap>();

    // Register `NamespaceSoundMap`
    app.register_type::<NamespaceSoundMap>()
        .register_type::<Handle<NamespaceSoundMap>>()
        .register_type_data::<Handle<NamespaceSoundMap>, ReflectHandle>()
        .init_asset::<NamespaceSoundMap>();

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
