use std::sync::atomic::{AtomicBool, Ordering};

use bevy::{prelude::*, utils::HashMap};
use froglight_core::data::ResourceKey;
use parking_lot::RwLock;

use crate::ResourcePack;

/// A manager for resource packs and their assets.
#[derive(Debug, Reflect)]
pub struct AssetManagerInner {
    #[reflect(ignore)]
    /// A list of loaded resource packs.
    pub handles: RwLock<Vec<Handle<ResourcePack>>>,

    /// A collection of loaded texture assets.
    #[reflect(ignore)]
    pub texture_assets: RwLock<HashMap<ResourceKey, Handle<Image>>>,

    /// A collection of loaded audio assets.
    #[reflect(ignore)]
    pub audio_assets: RwLock<HashMap<ResourceKey, Handle<AudioSource>>>,
}

static MANAGER_CREATED: AtomicBool = AtomicBool::new(false);

impl Default for AssetManagerInner {
    fn default() -> Self {
        // Log a warning if more than one `AssetManagerInner` is created.
        if MANAGER_CREATED.load(Ordering::Relaxed) {
            warn!("AssetManagerInner::default() called more than once!");
        } else {
            MANAGER_CREATED.store(true, Ordering::Relaxed);
        }

        Self {
            handles: RwLock::new(Vec::with_capacity(2)),
            texture_assets: RwLock::new(HashMap::with_capacity(512)),
            audio_assets: RwLock::new(HashMap::with_capacity(512)),
        }
    }
}
