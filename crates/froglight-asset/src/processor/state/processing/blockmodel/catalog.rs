use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_ecs::{
    prelude::not,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut},
};
use bevy_log::{debug, error};

use super::BlockModelProcessor;
use crate::{
    assets::raw::BlockModelDefinition, AssetCatalog, AssetProcess, ResourcePack, ResourcePackList,
};

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.add_systems(
        Update,
        BlockModelProcessor::catalog_blockmodel_definitions
            .run_if(not(BlockModelProcessor::is_model_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );
}

impl BlockModelProcessor {
    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`BlockModelProcessor`] is finished cataloging models.
    #[must_use]
    pub(super) fn is_model_finished(res: Res<Self>) -> bool { res.model_finished }

    /// A [`System`](bevy_ecs::system::System) that adds block model definitions
    /// to the [`AssetCatalog`] in batches.
    ///
    /// [`ResourcePack`]s are processed in the same order as they are in the
    /// [`ResourcePackList`].
    pub(super) fn catalog_blockmodel_definitions(
        resources: Res<ResourcePackList>,
        mut assets: ResMut<Assets<ResourcePack>>,
        mut catalog: ResMut<AssetCatalog>,
        mut state: ResMut<Self>,
    ) {
        let _ = Self::catalog_model_definitions_batch(
            &resources,
            &mut assets,
            &mut catalog,
            &mut state,
        );

        // Check if we've finished cataloging all model definitions
        if state.resource_index >= resources.len() {
            #[cfg(debug_assertions)]
            bevy_log::info!("BlockModelProcessor: Finished Cataloging");
            debug!(
                "BlockModelProcessor: Cataloged {} BlockModel Definitions",
                catalog.len_of::<BlockModelDefinition>()
            );
            state.model_finished = true;
        }
    }

    /// The number of models to catalog per frame.
    const CATALOGED_MODELS_PER_FRAME: usize = 50;

    fn catalog_model_definitions_batch(
        resources: &ResourcePackList,
        assets: &mut Assets<ResourcePack>,
        catalog: &mut AssetCatalog,
        state: &mut Self,
    ) -> Result<(), ()> {
        // Get the current ResourcePack.
        let handle = resources.get(state.resource_index).ok_or(())?;
        let asset = assets.get_mut(handle).ok_or_else(|| {
            error!("BlockModelProcessor: ResourcePack Asset missing!");
            state.resource_index += 1;
        })?;

        // Iterate over the next `CATALOGED_MODELS_PER_FRAME` sounds.
        let mut typed_catalog = catalog.typed_mut::<BlockModelDefinition>();
        for (model_key, model_handle) in asset
            .block_models
            .iter_mut()
            .skip(state.model_index)
            .take(Self::CATALOGED_MODELS_PER_FRAME)
        {
            // Replace the existing strong handle with a weak handle.
            let model_handle = std::mem::replace(model_handle, model_handle.clone_weak());

            // Add the taken strong handle to the catalog, if it doesn't already exist.
            typed_catalog.entry(model_key.to_owned()).or_insert(model_handle.untyped());

            // Increment the sound index.
            state.model_index += 1;
        }

        // If the model def index is at the end of the model list,
        // increment the resource index.
        if state.model_index >= asset.block_models.len() {
            state.resource_index += 1;
            state.model_index = 0;
        }

        Ok(())
    }
}
