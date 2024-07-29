use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::mesh::Mesh;
use bevy_state::state::OnEnter;

use super::resource_atlas::ResourceAtlasState;
use crate::{
    assets::{
        processed::{
            block_model::{BlockDataStorage, BlockModelStorage},
            BlockModel,
        },
        unprocessed::BlockModelDefinition,
    },
    AssetCatalog, AssetLoadState, ResourcePack, ResourcePackList,
};

mod generate;
mod recurse;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<BlockModelState>();

    // Reset the `BlockModelState` when entering `AssetLoadState::Processing`
    app.add_systems(OnEnter(AssetLoadState::Processing), BlockModelState::reset);

    // Generate `BlockModels`s from the `ResourcePackList`
    app.add_systems(
        Update,
        BlockModelState::create_block_models
            .ambiguous_with_all()
            .run_if(not(BlockModelState::is_finished))
            .run_if(ResourceAtlasState::is_finished)
            .after(ResourceAtlasState::create_resource_atlases)
            .in_set(AssetLoadState::Processing),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Resource)]
#[reflect(Default, Resource)]
pub(super) struct BlockModelState {
    resource_index: usize,
    model_index: usize,
    finished: bool,
}

impl BlockModelState {
    /// The number of [`BlockModel`]s to add to the [`AssetCatalog`] per frame.
    const BLOCKMODELS_PER_FRAME: usize = 8;

    /// Returns `true` if the [`BlockModelState`] has finished.
    pub(super) const fn finished(&self) -> bool { self.finished }

    /// Returns `true` if the [`BlockModelState`] has finished.
    fn is_finished(res: Res<Self>) -> bool { res.finished() }

    /// Create [`BlockModel`]s from the [`ResourcePackList`]s.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn create_block_models(
        list: Res<ResourcePackList>,
        packs: Res<Assets<ResourcePack>>,
        definitions: Res<Assets<BlockModelDefinition>>,

        mut state: ResMut<Self>,
        mut catalog: ResMut<AssetCatalog>,
        mut storage: ResMut<BlockDataStorage>,
        mut models: ResMut<Assets<BlockModel>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut handles: ResMut<BlockModelStorage>,
    ) {
        if Self::catalog_block_definitions(&list, &packs, &mut state, &mut catalog) {
            Self::generate_block_models(
                &definitions,
                &mut models,
                &mut meshes,
                &mut catalog,
                &mut storage,
                &mut handles,
            );
            state.finished = true;
        }
    }

    fn catalog_block_definitions(
        list: &ResourcePackList,
        packs: &Assets<ResourcePack>,

        state: &mut Self,
        catalog: &mut AssetCatalog,
    ) -> bool {
        let handle = list.get(state.resource_index).expect("ResourceIndex out of bounds");
        let resource = packs.get(handle).expect("ResourcePack not found");

        for (key, handle) in
            resource.block_models.iter().skip(state.model_index).take(Self::BLOCKMODELS_PER_FRAME)
        {
            catalog.entry::<BlockModelDefinition>(key.clone()).or_insert(handle.id().untyped());
            state.model_index += 1;
        }

        match (
            state.resource_index >= list.len().checked_sub(1).unwrap_or_default(),
            state.model_index >= resource.block_models.len().checked_sub(1).unwrap_or_default(),
        ) {
            (true, true) => {
                #[cfg(debug_assertions)]
                {
                    bevy_log::info!("AssetCatalog: Finished Cataloging BlockModels");
                    bevy_log::debug!(
                        "AssetCatalog: {} BlockModels",
                        catalog.len_of::<BlockModelDefinition>()
                    );
                }

                // We're done cataloging definitions, generate models
                return true;
            }
            (false, true) => {
                state.resource_index += 1;
                state.model_index = 0;
            }
            _ => {}
        }

        // We're not done cataloging definitions yet
        false
    }

    /// Resets the [`BlockModelState`].
    fn reset(mut res: ResMut<Self>) {
        res.resource_index = 0;
        res.model_index = 0;
        res.finished = false;
    }
}
