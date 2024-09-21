use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_ecs::{
    prelude::not,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut},
};
use bevy_log::debug;
use bevy_render::mesh::Mesh;
use bevy_transform::components::Transform;
use froglight_common::{Direction, ResourceKey};

use super::BlockModelProcessor;
use crate::{
    assets::{
        processed::{model::ModelTransformIndex, BlockAtlas, BlockModel, BlockModelCache},
        raw::{blockstate::StateModelDefinitions, BlockModelDefinition, BlockStateDefinition},
    },
    processor::state::BlockStateProcessor,
    AssetCatalog, AssetProcess,
};

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.add_systems(
        Update,
        BlockModelProcessor::create_blockmodels
            .after(BlockStateProcessor::catalog_blockstates)
            .after(BlockModelProcessor::create_block_atlas)
            .run_if(BlockStateProcessor::is_finished)
            .run_if(BlockModelProcessor::is_model_finished)
            .run_if(BlockModelProcessor::is_atlas_finished)
            .run_if(not(BlockModelProcessor::is_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );
}

impl BlockModelProcessor {
    /// The number of models to create per frame.
    const CREATED_MODELS_PER_FRAME: usize = 20;

    /// A [`System`](bevy_ecs::system::System) that builds [`BlockModel`]s from
    /// [`BlockStateDefinition`]s.
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::too_many_arguments)]
    pub fn create_blockmodels(
        states: Res<Assets<BlockStateDefinition>>,
        definitions: Res<Assets<BlockModelDefinition>>,
        cache: ResMut<BlockModelCache>,
        atlas: Res<BlockAtlas>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut models: ResMut<Assets<BlockModel>>,
        mut catalog: ResMut<AssetCatalog>,
        mut state: ResMut<Self>,
    ) {
        catalog.typed_mut_scope::<BlockModel>(|catalog, mut catalog_models| {
            for (_state_key, state_handle) in catalog
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
                                let Some(definition_key) =
                                    ResourceKey::try_new(def.model.clone()).ok()
                                else {
                                    continue;
                                };
                                let Some(definition_handle) =
                                    catalog.get_untyped::<BlockModelDefinition>(&definition_key)
                                else {
                                    continue;
                                };
                                let Some(definition) =
                                    definitions.get(definition_handle.id().typed_debug_checked())
                                else {
                                    continue;
                                };

                                if !catalog_models.contains(&definition_key) {
                                    let model = Self::create_model(
                                        definition,
                                        &definition_key,
                                        &definitions,
                                        &cache,
                                        &atlas,
                                        catalog,
                                        &mut meshes,
                                    );
                                    let model_handle = models.add(model);
                                    catalog_models.insert(definition_key, model_handle);
                                }
                            }
                        }
                        BlockStateDefinition::MultiPart { multipart } => {
                            for def in multipart.iter().flat_map(|p| p.apply.as_slice()) {
                                let Some(definition_key) =
                                    ResourceKey::try_new(def.model.clone()).ok()
                                else {
                                    continue;
                                };
                                let Some(definition_handle) =
                                    catalog.get_untyped::<BlockModelDefinition>(&definition_key)
                                else {
                                    continue;
                                };
                                let Some(definition) =
                                    definitions.get(definition_handle.id().typed_debug_checked())
                                else {
                                    continue;
                                };

                                if !catalog_models.contains(&definition_key) {
                                    let model = Self::create_model(
                                        definition,
                                        &definition_key,
                                        &definitions,
                                        &cache,
                                        &atlas,
                                        catalog,
                                        &mut meshes,
                                    );
                                    let model_handle = models.add(model);
                                    catalog_models.insert(definition_key, model_handle);
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
}

impl BlockModelProcessor {
    fn create_model(
        def: &BlockModelDefinition,
        def_key: &ResourceKey,
        defs: &Assets<BlockModelDefinition>,
        cache: &BlockModelCache,
        atlas: &BlockAtlas,
        catalog: &AssetCatalog,
        meshes: &mut Assets<Mesh>,
    ) -> BlockModel {
        // Get the ambient occlusion value
        let _ambient_occlusion = Self::get_ambient_occlusion(def, catalog, defs);

        // Get the display transforms for each display type
        let mut transforms = [Transform::default(); 8];
        for display_type in ModelTransformIndex::iter() {
            if let Some(display_transform) =
                Self::get_display_type(display_type, def, catalog, defs)
            {
                transforms[usize::from(display_type)] = display_transform.into();
            }
        }

        // Initialize the cache array
        let mut cache = cache.write();
        let cache = cache.entry(def_key.clone()).or_insert([
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
        ]);

        // Iterate over the elements and create meshes
        let mut block_mesh = Self::default_mesh();
        if let Some(elements) = Self::get_elements(def, catalog, defs) {
            for element in elements {
                // Append per-face data to the face meshes
                for (direction, face) in
                    Direction::iter().filter_map(|d| element.faces.get(&d).map(|f| (d, f)))
                {
                    let face_mesh = &mut cache[usize::from(direction)];
                    *face_mesh = Self::create_face_mesh(face, element, direction);

                    // Append the element positions to the direction mesh
                    Self::append_element_positions(face, element, face_mesh);

                    // Get the texture for the element face
                    let texture = Self::get_element_texture(face, def, def_key, catalog, defs);

                    // Append the element normals to the direction mesh
                    Self::append_element_normals(face, element, texture, catalog, face_mesh);

                    // Append the element uvs to the direction mesh
                    Self::append_element_uvs(
                        face, element, texture, atlas, catalog, direction, face_mesh,
                    );

                    // Append the direction mesh to the block mesh
                    block_mesh.merge(face_mesh);
                }
            }
        }

        BlockModel { block_mesh: meshes.add(block_mesh), transforms }
    }
}
