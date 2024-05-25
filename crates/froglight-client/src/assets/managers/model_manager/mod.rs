//! [`ModelManager`]
//!
//! Holds models for blocks, items, and entities.

use std::sync::Arc;

use bevy::prelude::*;
use froglight_assets::assets::{ModelDefinition as BlockItemDefinition, ResourcePack};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;
use parking_lot::RwLock;

mod block_item;
pub use block_item::*;

mod entity;
pub use entity::*;

use super::{AssetManager, LanguageManager, ParticleManager, SoundManager};
use crate::assets::{AssetLoading, ResourcePackSettings};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<ModelManager>()
        .register_type::<ModelManager>()
        .init_resource::<ModelManagerState>()
        .register_type::<ModelManagerState>();

    app.add_systems(
        OnEnter(AssetLoading::Loading),
        ModelManager::reset_model_manager.run_if(resource_exists::<ModelManager>),
    );
    app.add_systems(
        Update,
        ModelManager::populate_model_manager
            .run_if(not(ModelManager::is_finished))
            .run_if(resource_exists::<ModelManager>)
            .ambiguous_with(AssetManager::populate_asset_manager)
            .ambiguous_with(LanguageManager::populate_language_manager)
            .ambiguous_with(ParticleManager::populate_particle_manager)
            .ambiguous_with(SoundManager::populate_sound_manager)
            .in_set(AssetLoading::Processing),
    );
}

/// A [`Resource`] for managing model assets.
#[derive(Debug, Default, Clone, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct ModelManager {
    /// Block and Item Definitions
    block_item_defs: HashMap<ResourceKey, BlockItemDefinition>,
    /// Block and Item Models
    #[reflect(ignore)]
    pub block_item: BlockItemModels,

    /// Entity Models
    pub entities: HashMap<ResourceKey, EntityModel>,
}

/// Block and Item models stored in a [`HashMap`].
pub type BlockItemModels = Arc<RwLock<HashMap<ResourceKey, BlockItemModel>>>;

/// A [`Resource`] for managing the state of the [`ModelManager`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Reflect)]
#[reflect(Default, Resource)]
#[allow(unreachable_pub)]
pub struct ModelManagerState {
    finished: bool,
    current: usize,
}

impl ModelManager {
    /// Returns `true` if the [`ModelManager`] has finished loading all model
    /// definitions.
    #[must_use]
    pub fn is_finished(state: Res<ModelManagerState>) -> bool { state.finished }

    /// Resets the [`ModelManager`] to its initial state.
    fn reset_model_manager(
        mut manager: ResMut<ModelManager>,
        mut state: ResMut<ModelManagerState>,
    ) {
        manager.block_item_defs.clear();
        manager.block_item.write().clear();
        manager.entities.clear();
        state.finished = false;
        state.current = 0;
    }

    /// Populates the [`ModelManager`] with model definitions from the currently
    /// loaded [`ResourcePack`]s.
    ///
    /// This does not create any models, it only collects the definitions.
    ///
    /// Does not rely on any other asset managers.
    pub fn populate_model_manager(
        settings: Res<ResourcePackSettings>,
        mut assets: ResMut<Assets<ResourcePack>>,
        mut manager: ResMut<ModelManager>,
        mut state: ResMut<ModelManagerState>,
    ) {
        // Get the current `ResourcePack` from the list
        if let Some(pack_item) = settings.resourcepacks.get(state.current) {
            // If the `ResourcePack` has a handle
            if let Some(pack_handle) = pack_item.handle.as_ref() {
                // Access the `ResourcePack` data
                if let Some(resourcepack) = assets.get_mut(pack_handle) {
                    // Take the models from the `ResourcePack`,
                    // if they don't already exist.
                    for (resourcekey, sound_handle) in std::mem::take(&mut resourcepack.models) {
                        manager.block_item_defs.entry(resourcekey).or_insert(sound_handle);
                    }
                } else if let Some(path) = &pack_item.path {
                    error!("Failed to access ResourcePack: \"{path}\"");
                } else {
                    error!("Failed to access ResourcePack: #{}", state.current);
                }
            }
        }

        // Increment the current `ResourcePack` index
        state.current += 1;

        // Set the finished flag if all `ResourcePack`s have been loaded
        if state.current >= settings.resourcepacks.len() {
            #[cfg(debug_assertions)]
            debug!("Loaded \"{}\" block and item model definitions", manager.block_item_defs.len());
            state.finished = true;
        }
    }
}
