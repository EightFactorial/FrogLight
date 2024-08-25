use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Commands, Res, ResMut, Resource},
};
use bevy_log::{debug, error};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::texture::Image;
use bevy_sprite::{TextureAtlasBuilder, TextureAtlasLayout};
use bevy_state::state::OnEnter;
use froglight_common::ResourceKey;

use super::{BlockStateProcessor, TextureProcessor};
use crate::{
    assets::{
        processed::{model::BlockModel, BlockAtlas},
        raw::{
            blockstate::{StateModelDefinition, StateModelDefinitions},
            model::ResourceOrVariable,
            BlockModelDefinition, BlockStateDefinition,
        },
    },
    AssetCatalog, AssetProcess, ResourcePack, ResourcePackList,
};

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.register_type::<BlockModelProcessor>();
    app.init_resource::<BlockModelProcessor>();

    // Reset the `BlockModelProcessor` state
    app.add_systems(OnEnter(AssetProcess::Processing), BlockModelProcessor::reset_blockmodel_state);
    // Clear the `AssetCatalog` blockmodels
    app.add_systems(OnEnter(AssetProcess::Processing), BlockModelProcessor::clear_catalog_models);

    // Catalog BlockModelDefinitions
    app.add_systems(
        Update,
        BlockModelProcessor::catalog_blockmodel_definitions
            .run_if(not(BlockModelProcessor::is_model_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );

    // Create BlockAtlas
    app.add_systems(
        Update,
        BlockModelProcessor::create_block_atlas
            .after(TextureProcessor::catalog_textures)
            .after(BlockModelProcessor::catalog_blockmodel_definitions)
            .run_if(TextureProcessor::is_finished)
            .run_if(BlockModelProcessor::is_model_finished)
            .run_if(not(BlockModelProcessor::is_atlas_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );

    // Create BlockModels
    app.add_systems(
        Update,
        BlockModelProcessor::create_blockmodels
            .after(BlockStateProcessor::catalog_blockstates)
            .after(BlockModelProcessor::create_block_atlas)
            .run_if(BlockStateProcessor::is_finished)
            .run_if(BlockModelProcessor::is_atlas_finished)
            .run_if(not(BlockModelProcessor::is_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );
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

    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`BlockModelProcessor`] is finished cataloging models.
    #[must_use]
    fn is_model_finished(res: Res<Self>) -> bool { res.model_finished }

    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`BlockModelProcessor`] is has created the [`BlockAtlas`].
    #[must_use]
    fn is_atlas_finished(res: Res<Self>) -> bool { res.atlas_finished }

    /// A [`System`](bevy_ecs::system::System) that adds block model definitions
    /// to the [`AssetCatalog`] in batches.
    ///
    /// [`ResourcePack`]s are processed in the same order as they are in the
    /// [`ResourcePackList`].
    fn catalog_blockmodel_definitions(
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

    /// Creates a [`BlockAtlas`] from all textures referenced in
    /// [`BlockModelDefinition`]s.
    fn create_block_atlas(
        definitions: Res<Assets<BlockModelDefinition>>,
        catalog: Res<AssetCatalog>,
        mut images: ResMut<Assets<Image>>,
        mut atlases: ResMut<Assets<TextureAtlasLayout>>,
        mut state: ResMut<Self>,
        mut commands: Commands,
    ) {
        let mut builder = TextureAtlasBuilder::default();
        builder.initial_size((512, 512).into()).max_size((4096, 4096).into());

        // Iterate over all `BlockModelDefinition`s
        for def in catalog
            .typed_ref::<BlockModelDefinition>()
            .unwrap()
            .iter_untyped()
            .filter_map(|(_, h)| definitions.get(h.id().typed_debug_checked()))
        {
            // If the `BlockModelDefinition` has textures
            if let Some(textures) = &def.textures {
                // Get all texture references
                for texture in textures.values().filter_map(|v| {
                    if let ResourceOrVariable::Resource(key) = v {
                        ResourceKey::try_new(key).ok()
                    } else {
                        None
                    }
                }) {
                    // Get the texture from the catalog
                    if let Some((Some(image), handle)) = catalog
                        .typed_ref::<Image>()
                        .unwrap()
                        .get(&texture)
                        .map(|h| (images.get(&h), h))
                    {
                        // Add the texture to the atlas
                        builder.add_texture(Some(handle.id()), image);
                    }
                }
            }
        }

        // Build the `BlockAtlas`
        match builder.build() {
            Ok((atlas, image)) => {
                let atlas_handle = atlases.add(atlas.clone());
                let image_handle = images.add(image);
                commands.insert_resource(BlockAtlas::new(atlas, atlas_handle, image_handle));
            }
            Err(err) => {
                error!("BlockModelProcessor: Failed to build BlockAtlas, {err}");
            }
        }

        debug!("BlockModelProcessor: Created BlockAtlas");
        state.atlas_finished = true;
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

    /// The number of models to create per frame.
    const CREATED_MODELS_PER_FRAME: usize = 20;

    /// A [`System`](bevy_ecs::system::System) that builds [`BlockModel`]s from
    /// [`BlockStateDefinition`]s.
    #[allow(clippy::missing_panics_doc)]
    pub fn create_blockmodels(
        states: Res<Assets<BlockStateDefinition>>,
        mut models: ResMut<Assets<BlockModel>>,
        mut catalog: ResMut<AssetCatalog>,
        mut state: ResMut<Self>,
    ) {
        catalog.typed_mut_scope::<BlockModel>(|catalog, mut catalog_models| {
            for (state_key, state_handle) in catalog
                .typed_ref::<BlockStateDefinition>()
                .unwrap()
                .iter_untyped()
                .skip(state.state_index)
                .take(Self::CREATED_MODELS_PER_FRAME)
            {
                state.state_index += 1;

                if let Some(state) = states.get(state_handle.id().typed_debug_checked()) {
                    match state {
                        BlockStateDefinition::Variants { variants } => {
                            for def in variants.values().flat_map(StateModelDefinitions::as_slice) {
                                if let Some(model) = Self::create_model(def, catalog) {
                                    let model_handle = models.add(model);
                                    catalog_models.insert(state_key.clone(), model_handle);
                                }
                            }
                        }
                        BlockStateDefinition::MultiPart { multipart } => {
                            for def in multipart.iter().flat_map(|p| p.apply.as_slice()) {
                                if let Some(model) = Self::create_model(def, catalog) {
                                    let model_handle = models.add(model);
                                    catalog_models.insert(state_key.clone(), model_handle);
                                }
                            }
                        }
                    }
                }
            }
        });

        // Check if we've finished processing all blockstates
        if state.state_index >= catalog.len_of::<BlockStateDefinition>() {
            #[cfg(debug_assertions)]
            bevy_log::info!("BlockModelProcessor: Finished");
            debug!("BlockModelProcessor: Created {} BlockModels", catalog.len_of::<BlockModel>());

            *state = Self {
                model_finished: true,
                atlas_finished: true,
                finished: true,
                ..Self::default()
            };
        }
    }

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

impl BlockModelProcessor {
    fn create_model(def: &StateModelDefinition, _catalog: &AssetCatalog) -> Option<BlockModel> {
        let _model_key = ResourceKey::try_new(def.model.clone()).ok()?;
        None
    }
}
