use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_log::{debug, error};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_state::state::OnEnter;

use crate::{
    assets::raw::BlockStateDefinition, AssetCatalog, AssetProcess, ResourcePack, ResourcePackList,
};

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.register_type::<BlockStateProcessor>();
    app.init_resource::<BlockStateProcessor>();

    // Reset the `BlockStateProcessor` state
    app.add_systems(OnEnter(AssetProcess::Processing), BlockStateProcessor::reset_blockstate_state);
    // Clear the `AssetCatalog` blockstates
    app.add_systems(
        OnEnter(AssetProcess::Processing),
        BlockStateProcessor::clear_catalog_blockstates,
    );

    // Catalog blockstates
    app.add_systems(
        Update,
        BlockStateProcessor::catalog_blockstates
            .run_if(not(BlockStateProcessor::is_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );
}

/// A processor that catalogs textures in the [`AssetCatalog`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct BlockStateProcessor {
    resource_index: usize,
    blockstate_index: usize,
    finished: bool,
}

impl BlockStateProcessor {
    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`TextureProcessor`] is finished.
    #[must_use]
    pub fn is_finished(res: Res<Self>) -> bool { res.finished }

    /// A [`System`](bevy_ecs::system::System) that adds textures to the
    /// [`AssetCatalog`] in batches.
    ///
    /// [`ResourcePack`]s are processed in the same order as they are in the
    /// [`ResourcePackList`].
    pub fn catalog_blockstates(
        resources: Res<ResourcePackList>,
        mut assets: ResMut<Assets<ResourcePack>>,
        mut catalog: ResMut<AssetCatalog>,
        mut state: ResMut<Self>,
    ) {
        let _ = Self::catalog_blockstate_batch(&resources, &mut assets, &mut catalog, &mut state);

        // Check if the processor is finished.
        if state.resource_index >= resources.len() {
            #[cfg(debug_assertions)]
            bevy_log::info!("BlockStateProcessor: Finished");
            debug!(
                "BlockStateProcessor: Cataloged {} BlockState Definitions",
                catalog.len_of::<BlockStateDefinition>()
            );
            // Set the processor to finished.
            *state = Self { finished: true, ..Self::default() };
        }
    }

    /// The number of blockstates to process per frame.
    const BLOCKSTATES_PER_FRAME: usize = 50;

    /// Catalogs a batch of blockstates.
    fn catalog_blockstate_batch(
        resources: &ResourcePackList,
        assets: &mut Assets<ResourcePack>,
        catalog: &mut AssetCatalog,
        state: &mut BlockStateProcessor,
    ) -> Result<(), ()> {
        // Get the current ResourcePack.
        let handle = resources.get(state.resource_index).ok_or(())?;
        let asset = assets.get_mut(handle).ok_or_else(|| {
            error!("BlockStateProcessor: ResourcePack Asset missing!");
            state.resource_index += 1;
        })?;

        // Iterate over the next `BLOCKSTATES_PER_FRAME` textures.
        let mut typed_catalog = catalog.typed_mut::<BlockStateDefinition>();
        for (blockstate_key, blockstate_handle) in asset
            .block_states
            .iter_mut()
            .skip(state.blockstate_index)
            .take(Self::BLOCKSTATES_PER_FRAME)
        {
            // Replace the existing strong handle with a weak handle.
            let blockstate_handle =
                std::mem::replace(blockstate_handle, blockstate_handle.clone_weak());

            // Add the taken strong handle to the catalog, if it doesn't already exist.
            typed_catalog.entry(blockstate_key.to_owned()).or_insert(blockstate_handle.untyped());

            // Increment the blockstate index.
            state.blockstate_index += 1;
        }

        // If the blockstate index is at the end of the state list,
        // increment the resource index.
        if state.blockstate_index >= asset.block_states.len() {
            state.resource_index += 1;
            state.blockstate_index = 0;
        }

        Ok(())
    }

    /// Resets the state of the [`BlockStateProcessor`].
    fn reset_blockstate_state(mut res: ResMut<Self>) {
        #[cfg(debug_assertions)]
        bevy_log::trace!("BlockStateProcessor: Resetting state");
        *res = Self::default();
    }

    /// Clears all textures from the [`AssetCatalog`].
    fn clear_catalog_blockstates(mut catalog: ResMut<AssetCatalog>) {
        #[cfg(debug_assertions)]
        bevy_log::info!("BlockStateProcessor: Clearing AssetCatalog BlockStates");
        catalog.clear_of::<BlockStateDefinition>();
        // catalog.clear_of::<BlockState>();
    }
}
