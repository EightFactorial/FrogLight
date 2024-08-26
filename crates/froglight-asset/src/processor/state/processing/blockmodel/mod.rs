use bevy_app::App;
use bevy_ecs::{
    reflect::ReflectResource,
    system::{Res, ResMut, Resource},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_state::state::OnEnter;

use crate::{
    assets::{processed::model::BlockModel, raw::BlockModelDefinition},
    AssetCatalog, AssetProcess,
};

mod atlas;
mod catalog;

mod model;
mod model_data;

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.register_type::<BlockModelProcessor>();
    app.init_resource::<BlockModelProcessor>();

    // Reset the `BlockModelProcessor` state
    app.add_systems(OnEnter(AssetProcess::Processing), BlockModelProcessor::reset_blockmodel_state);
    // Clear the `AssetCatalog` blockmodels
    app.add_systems(OnEnter(AssetProcess::Processing), BlockModelProcessor::clear_catalog_models);

    atlas::build(app);
    catalog::build(app);
    model::build(app);
}

/// A processor that creates [`BlockModel`] for [`BlockStateDefinition`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct BlockModelProcessor {
    resource_index: usize,
    model_index: usize,
    model_finished: bool,

    atlas_finished: bool,

    state_index: usize,
    finished: bool,
}

impl BlockModelProcessor {
    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`BlockModelProcessor`] is finished.
    #[must_use]
    pub fn is_finished(res: Res<Self>) -> bool { res.finished }

    /// Resets the state of the [`BlockModelProcessor`].
    fn reset_blockmodel_state(mut res: ResMut<Self>) {
        #[cfg(debug_assertions)]
        bevy_log::trace!("BlockModelProcessor: Resetting state");
        *res = Self::default();
    }

    /// Clears the [`AssetCatalog`] of all [`BlockModel`]s.
    fn clear_catalog_models(mut catalog: ResMut<AssetCatalog>) {
        #[cfg(debug_assertions)]
        bevy_log::info!("BlockModelProcessor: Clearing AssetCatalog BlockModels");
        catalog.clear_of::<BlockModelDefinition>();
        catalog.clear_of::<BlockModel>();
    }
}
