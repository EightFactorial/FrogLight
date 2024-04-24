use bevy_asset::Handle;
use bevy_audio::AudioSource;
use bevy_render::texture::Image;
use froglight_protocol::common::ResourceKey;
use hashbrown::HashMap;
use parking_lot::RwLock;

use super::{blockmap::BlockMap, soundmap::SoundMap};
use crate::assets::resourcepack::ResourcePack;

#[derive(Debug, Default)]
pub struct AssetManagerInner {
    /// All loaded resource packs.
    pub resourcepacks: RwLock<Vec<Handle<ResourcePack>>>,

    /// All loaded textures.
    pub textures: RwLock<HashMap<ResourceKey, Handle<Image>>>,
    /// All loaded audio files.
    pub audio: RwLock<HashMap<ResourceKey, Handle<AudioSource>>>,

    /// Block definitions.
    pub blocks: RwLock<BlockMap>,
    /// Sound definitions.
    pub sounds: RwLock<SoundMap>,
}
