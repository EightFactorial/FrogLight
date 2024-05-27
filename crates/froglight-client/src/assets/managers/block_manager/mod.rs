use std::sync::Arc;

use bevy::prelude::*;
use froglight_assets::assets::{BlockStateDefinition, ResourcePack};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;
use parking_lot::RwLock;

use super::{AssetManager, LanguageManager, ParticleManager, SoundManager};
use crate::assets::{AssetLoading, ModelManager, ResourcePackSettings};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<BlockManager>()
        .register_type::<BlockManager>()
        .init_resource::<BlockManagerState>()
        .register_type::<BlockManagerState>();

    app.add_systems(
        OnEnter(AssetLoading::Loading),
        BlockManager::reset_block_manager.run_if(resource_exists::<BlockManager>),
    );
    app.add_systems(
        Update,
        BlockManager::populate_block_manager
            .run_if(not(BlockManager::is_pack_finished))
            .run_if(resource_exists::<BlockManager>)
            .ambiguous_with(AssetManager::populate_asset_manager)
            .ambiguous_with(LanguageManager::populate_language_manager)
            .ambiguous_with(ModelManager::populate_model_manager)
            .ambiguous_with(ParticleManager::populate_particle_manager)
            .ambiguous_with(SoundManager::populate_sound_manager)
            .in_set(AssetLoading::Processing),
    );
    app.add_systems(
        Update,
        BlockManager::process_blockstates
            .run_if(AssetManager::is_finished)
            .run_if(BlockManager::is_pack_finished)
            .run_if(not(BlockManager::is_finished))
            .run_if(resource_exists::<BlockManager>)
            .after(AssetManager::populate_asset_manager)
            .after(BlockManager::populate_block_manager)
            .after(ModelManager::populate_model_manager)
            .in_set(AssetLoading::Processing),
    );
}

/// A [`Resource`] for managing the blocks loaded in the game.
///
/// Can safely be cloned and shared between threads.
#[derive(Debug, Default, Clone, Resource, Deref, DerefMut, Reflect)]
#[reflect(Default, Resource)]
pub struct BlockManager(#[reflect(ignore)] Arc<RwLock<BlockManagerInner>>);

#[derive(Debug, Default, Reflect)]
pub struct BlockManagerInner {
    blockstate_defs: HashMap<ResourceKey, BlockStateDefinition>,
    pub blockstates: HashMap<ResourceKey, ()>,
}

/// The state of the [`BlockManager`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Reflect)]
#[reflect(Default, Resource)]
#[allow(unreachable_pub)]
pub struct BlockManagerState {
    pub block_finished: bool,
    pub block_current: usize,

    pub pack_finished: bool,
    pub pack_current: usize,

    #[cfg(debug_assertions)]
    instant: std::time::Instant,
}

impl Default for BlockManagerState {
    fn default() -> Self {
        Self {
            block_finished: false,
            block_current: 0,

            pack_finished: false,
            pack_current: 0,

            #[cfg(debug_assertions)]
            instant: std::time::Instant::now(),
        }
    }
}

impl BlockManager {
    /// Returns `true` if the [`BlockManager`] has finished loading.
    #[must_use]
    pub fn is_finished(state: Res<BlockManagerState>) -> bool {
        state.block_finished && state.pack_finished
    }

    /// Returns `true` if the [`BlockManager`] has finished loading all block
    /// states.
    #[must_use]
    pub fn is_block_finished(state: Res<BlockManagerState>) -> bool { state.block_finished }

    /// Returns `true` if the [`BlockManager`] has finished loading all block
    /// definitions.
    #[must_use]
    pub fn is_pack_finished(state: Res<BlockManagerState>) -> bool { state.pack_finished }

    /// Resets the [`BlockManager`] to its initial state.
    fn reset_block_manager(manager: ResMut<BlockManager>, mut state: ResMut<BlockManagerState>) {
        {
            let mut inner = manager.0.write();
            inner.blockstates.clear();
        }
        state.block_finished = false;
        state.pack_finished = false;
        state.block_current = 0;
        state.pack_current = 0;

        #[cfg(debug_assertions)]
        {
            state.instant = std::time::Instant::now();
        }
    }

    /// Populates the [`BlockManager`] with blocks from currently loaded assets.
    ///
    /// Does not rely on any other asset managers.
    pub(crate) fn populate_block_manager(
        settings: Res<ResourcePackSettings>,
        manager: ResMut<BlockManager>,
        mut assets: ResMut<Assets<ResourcePack>>,
        mut state: ResMut<BlockManagerState>,
    ) {
        // Get the current `ResourcePack` from the list
        if let Some(pack_item) = settings.resourcepacks.get(state.pack_current) {
            // If the `ResourcePack` has a handle
            if let Some(pack_handle) = pack_item.handle.as_ref() {
                // Access the `ResourcePack` data
                if let Some(resourcepack) = assets.get_mut(pack_handle) {
                    // Take the blockstates from the `ResourcePack`,
                    // if they don't already exist.
                    let mut manager = manager.write();
                    for (resourcekey, state_def) in std::mem::take(&mut resourcepack.blockstates) {
                        manager.blockstate_defs.entry(resourcekey).or_insert(state_def);
                    }
                } else if let Some(path) = &pack_item.path {
                    error!("Failed to access ResourcePack: \"{path}\"");
                } else {
                    error!("Failed to access ResourcePack: #{}", state.pack_current);
                }
            }
        }

        // Increment the current `ResourcePack` index
        state.pack_current += 1;

        // Set the finished flag if all `ResourcePack`s have been loaded
        if state.pack_current >= settings.resourcepacks.len() {
            #[cfg(debug_assertions)]
            debug!("Loaded \"{}\" blockstate definitions", manager.read().blockstate_defs.len());
            state.pack_finished = true;

            #[cfg(debug_assertions)]
            {
                state.instant = std::time::Instant::now();
            }
        }
    }

    /// The number of blockstates to process per frame.
    ///
    /// Higher values load faster, but may cause performance issues.
    const BLOCKSTATES_PER_FRAME: usize = 100;

    /// Processes the blockstates loaded in the [`BlockManager`].
    ///
    /// Loads [`Self::BLOCKSTATES_PER_FRAME`] blockstates per frame.
    ///
    /// Requires the [`AssetManager`] and [`ModelManager`] to be loaded.
    pub(crate) fn process_blockstates(
        manager: Res<BlockManager>,
        mut state: ResMut<BlockManagerState>,

        _asset_manager: Res<AssetManager>,
        model_manager: ResMut<ModelManager>,

        _mesh_assets: ResMut<Assets<Mesh>>,
    ) {
        let mut manager = manager.0.write();

        // Load `BLOCKSTATES_PER_FRAME` blockstates per frame
        {
            let mut _model_manager = model_manager.write();
            for (_state_key, _def) in manager
                .blockstate_defs
                .iter()
                .skip(state.block_current)
                .take(Self::BLOCKSTATES_PER_FRAME)
            {
                // #[cfg(debug_assertions)]
                // if model_manager
                //     .get_or_load_model(model_key, &asset_manager, &mut
                // mesh_assets)     .is_none()
                // {
                //     error!("Failed to load block model: \"{model_key}\"");
                // }

                // #[cfg(not(debug_assertions))]
                // model_manager.get_or_load_model(model_key, &asset_manager,
                // &mut mesh_assets);
            }
        }

        // Increment the current blockstate index
        state.block_current += Self::BLOCKSTATES_PER_FRAME;

        // Set the finished flag if all blockstates have been loaded
        if state.block_current >= manager.blockstate_defs.len() {
            state.block_finished = true;

            #[cfg(debug_assertions)]
            {
                debug!("Processed \"{}\" blockstates", manager.blockstate_defs.len());
                debug!("Processed blockstates in {:?}", state.instant.elapsed());
            }

            // Clear the blockstate and model definitions
            manager.blockstate_defs.clear();
            manager.blockstate_defs.shrink_to_fit();

            let mut model_manager = model_manager.write();
            model_manager.block_item_defs.clear();
            model_manager.block_item_defs.shrink_to_fit();
        }
    }
}
