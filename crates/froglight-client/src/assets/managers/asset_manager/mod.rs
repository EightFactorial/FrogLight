use bevy::prelude::*;
use froglight_assets::assets::{
    BlockStateDefinition, FontDefinition, ModelDefinition, ParticleDefinition, ResourcePack,
};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.init_resource::<AssetManager>(); }

/// A [`Resource`] for managing [`sound`](AudioSource) and [`texture`](Image)
/// assets.
#[derive(Debug, Default, Clone, Resource)]
pub struct AssetManager {
    /// Loaded [`ResourcePack`]s
    pub resourcepacks: Vec<Handle<ResourcePack>>,

    /// Blockstates
    /// TODO: `BlockStateManager`
    pub blockstates: HashMap<ResourceKey, BlockStateDefinition>,
    /// Fonts
    /// TODO: `FontManager`
    pub font: HashMap<ResourceKey, FontDefinition>,
    /// Models
    /// TODO: `ModelManager`
    pub models: HashMap<ResourceKey, ModelDefinition>,
    /// Particles
    /// TODO: `ParticleManager`
    pub particles: HashMap<ResourceKey, ParticleDefinition>,
    /// Sounds
    pub sounds: HashMap<ResourceKey, Handle<AudioSource>>,
    /// Textures
    pub textures: HashMap<ResourceKey, Handle<Image>>,
}

impl AssetManager {
    /// Clear all collected assets.
    pub(crate) fn clear(&mut self) {
        self.resourcepacks.clear();
        self.blockstates.clear();
        self.font.clear();
        self.models.clear();
        self.particles.clear();
        self.sounds.clear();
        self.textures.clear();
    }

    /// Take all assets from a [`ResourcePack`]
    /// and insert them into the [`AssetManager`](super::AssetManager).
    ///
    /// # Warning
    /// This will drain the [`ResourcePack`] of all assets
    /// and drop any assets that were not used.
    pub(crate) fn insert(&mut self, resourcepack: &mut ResourcePack) {
        // Insert blockstates
        for (key, value) in resourcepack.blockstates.drain() {
            self.blockstates.entry(key).or_insert(value);
        }
        // Insert fonts
        for (key, value) in resourcepack.font.drain() {
            self.font.entry(key).or_insert(value);
        }
        // Insert models
        for (key, value) in resourcepack.models.drain() {
            self.models.entry(key).or_insert(value);
        }
        // Insert particles
        for (key, value) in resourcepack.particles.drain() {
            self.particles.entry(key).or_insert(value);
        }
        // Insert sounds
        for (key, value) in resourcepack.sounds.drain() {
            self.sounds.entry(key).or_insert(value);
        }
        // Insert textures
        for (key, value) in resourcepack.textures.drain() {
            self.textures.entry(key).or_insert(value);
        }
    }
}
