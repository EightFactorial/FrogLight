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
///
/// Can safely be cloned and shared between threads.
#[derive(Debug, Default, Clone, Resource, Deref, Reflect)]
#[reflect(Default, Resource)]
pub struct ModelManager(#[reflect(ignore)] Arc<RwLock<ModelManagerInner>>);

/// The inner data of the [`ModelManager`].
#[derive(Debug, Default)]
pub struct ModelManagerInner {
    /// Block and Item Definitions
    pub(crate) block_item_defs: HashMap<ResourceKey, BlockItemDefinition>,
    /// Block and Item Models
    pub block_item: HashMap<ResourceKey, BlockItemModel>,

    /// Entity Models
    pub entities: HashMap<ResourceKey, EntityModel>,
}

impl ModelManagerInner {
    /// Loads a [`BlockItemModel`] into the [`ModelManager`].
    pub(crate) fn load_model(
        &mut self,
        key: &ResourceKey,
        asset_manager: &AssetManager,
        mesh_assets: &mut Assets<Mesh>,
    ) -> Option<&BlockItemModel> {
        self.block_item_defs.get(key).map(|def| {
            // Resolve the model definition into a model
            let model = BlockItemModel::resolve_definition(
                key,
                def,
                &self.block_item_defs,
                asset_manager,
                mesh_assets,
            );

            // Insert the model into the map and return a reference
            self.block_item.insert(key.clone(), model);
            self.block_item.get(key).unwrap()
        })
    }

    /// Returns the [`BlockItemModel`] for the given key, loading it if it
    /// doesn't already exist.
    pub fn get_or_load_model(
        &mut self,
        key: &ResourceKey,
        asset_manager: &AssetManager,
        mesh_assets: &mut Assets<Mesh>,
    ) -> Option<&BlockItemModel> {
        if self.block_item.contains_key(key) {
            self.block_item.get(key)
        } else {
            self.load_model(key, asset_manager, mesh_assets)
        }
    }
}

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
    fn reset_model_manager(manager: ResMut<ModelManager>, mut state: ResMut<ModelManagerState>) {
        {
            let mut inner = manager.write();
            inner.block_item_defs.clear();
            inner.block_item.clear();
            inner.entities.clear();
        }
        state.finished = false;
        state.current = 0;
    }

    /// Populates the [`ModelManager`] with model definitions from the currently
    /// loaded [`ResourcePack`]s.
    ///
    /// This does not create any models, it only collects the definitions.
    ///
    /// Does not rely on any other asset managers.
    pub(crate) fn populate_model_manager(
        settings: Res<ResourcePackSettings>,
        manager: ResMut<ModelManager>,
        mut assets: ResMut<Assets<ResourcePack>>,
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
                    let mut manager = manager.write();
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
            {
                let manager = manager.read();
                debug!(
                    "Loaded \"{}\" block and item model definitions",
                    manager.block_item_defs.len()
                );
                debug!("Loaded \"{}\" entity model definitions", manager.entities.len());
            }

            state.finished = true;
        }
    }
}
