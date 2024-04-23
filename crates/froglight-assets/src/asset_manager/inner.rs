use bevy_asset::Handle;
use bevy_audio::AudioSource;
use bevy_reflect::TypePath;
use bevy_render::texture::Image;
use froglight_protocol::common::ResourceKey;
use hashbrown::HashMap;
use parking_lot::RwLock;

#[derive(Debug, Default, TypePath)]
pub struct AssetManagerInner {
    /// All loaded textures.
    pub textures: RwLock<HashMap<ResourceKey, Handle<Image>>>,
    /// All loaded sounds.
    pub sounds: RwLock<HashMap<ResourceKey, Handle<AudioSource>>>,
}
