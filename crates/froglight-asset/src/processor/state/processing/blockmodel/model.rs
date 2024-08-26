use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_ecs::{
    prelude::not,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut},
};
use bevy_log::debug;
use bevy_math::prelude::Cuboid;
use bevy_render::mesh::{Mesh, VertexAttributeValues};
use bevy_transform::components::Transform;
use froglight_common::{Direction, ResourceKey};
use glam::{FloatExt, Vec3};

use super::BlockModelProcessor;
use crate::{
    assets::{
        processed::{
            model::{BlockModel, BlockModelCache, ModelTransformIndex},
            BlockAtlas,
        },
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
    // Suppose Y-up right hand, and camera look from +Z to -Z
    const CUBOID_FACES: [Direction; 6] = [
        // Front
        Direction::South,
        // Back
        Direction::North,
        // Right
        Direction::East,
        // Left
        Direction::West,
        // Top
        Direction::Up,
        // Bottom
        Direction::Down,
    ];

    fn create_model(
        definition: &BlockModelDefinition,
        definition_key: &ResourceKey,
        definitions: &Assets<BlockModelDefinition>,
        cache: &BlockModelCache,
        atlas: &BlockAtlas,
        catalog: &AssetCatalog,
        meshes: &mut Assets<Mesh>,
    ) -> BlockModel {
        // Get the ambient occlusion value
        let _ambient_occlusion = Self::get_ambient_occlusion(definition, catalog, definitions);

        // Get the display transforms for each display type
        let mut transforms = [Transform::default(); 8];
        for display_type in ModelTransformIndex::iter() {
            if let Some(display_transform) =
                Self::get_display_type(display_type, definition, catalog, definitions)
            {
                transforms[usize::from(display_type)] = display_transform.into();
            }
        }

        // Initialize the cache array
        let mut cache = cache.write();
        let cache = cache.entry(definition_key.clone()).or_insert([
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
        ]);

        // Iterate over the elements and create meshes
        let mut block_mesh = Self::default_mesh();
        if let Some(elements) = Self::get_elements(definition, catalog, definitions) {
            for element in elements {
                let (from, to): (Vec3, Vec3) = (element.from.into(), element.to.into());

                let mut element_mesh = Mesh::from(Cuboid::from_corners(from, to));
                element_mesh.translate_by(from.midpoint(to) - Vec3::splat(8.0));
                element_mesh.scale_by(Vec3::splat(1.0 / 16.0));

                // TODO: Rotate the element mesh based on the element rotation

                // Append per-face data to the directional meshes
                for direction in Self::CUBOID_FACES {
                    let Some(element_face) = element.faces.get(&direction) else {
                        continue;
                    };

                    let direction_mesh = &mut cache[usize::from(direction)];
                    let attribute_group = usize::from(direction);

                    // TODO: Set the element mesh UVs based on the element texture
                    // TODO: Apply the element face rotation to the UVs
                    if let Some(texture_handle) = Self::get_element_texture(
                        element_face,
                        definition,
                        definition_key,
                        catalog,
                        definitions,
                    ) {
                        if let Some(atlas_index) = atlas
                            .layout()
                            .get_texture_index(texture_handle.id().typed_debug_checked())
                        {
                            let atlas_rect = atlas.layout().textures[atlas_index].as_rect();
                            let atlas_size = atlas.layout().size.as_vec2();

                            let element_uvs =
                                element_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
                            let VertexAttributeValues::Float32x2(element_uvs) = element_uvs else {
                                unreachable!();
                            };

                            let _face_uvs = element_face.uv(element);

                            let uv_range = (attribute_group * 4)..(attribute_group * 4 + 4);
                            for [u, v] in &mut element_uvs[uv_range] {
                                *u = u.remap(
                                    0.0,
                                    1.0,
                                    atlas_rect.min.x / atlas_size.x,
                                    atlas_rect.max.x / atlas_size.x,
                                );
                                *v = v.remap(
                                    0.0,
                                    1.0,
                                    atlas_rect.min.y / atlas_size.y,
                                    atlas_rect.max.y / atlas_size.y,
                                );
                            }
                        } else {
                            #[cfg(debug_assertions)]
                            bevy_log::error!("BlockModelProcessor: BlockAtlas missing {direction:?} texture for \"{definition_key}\"");
                        }
                    } else {
                        #[cfg(debug_assertions)]
                        bevy_log::error!("BlockModelProcessor: AssetCatalog missing {direction:?} texture for \"{definition_key}\"");
                    }

                    // Append the element positions to the direction mesh
                    Self::append_element_positions(attribute_group, direction_mesh, &element_mesh);
                    // Append the element normals to the direction mesh
                    Self::append_element_normals(attribute_group, direction_mesh, &element_mesh);
                    // Append the element uvs to the direction mesh
                    Self::append_element_uvs(attribute_group, direction_mesh, &element_mesh);
                }

                block_mesh.merge(&element_mesh);
            }
        }

        BlockModel { block_mesh: meshes.add(block_mesh), transforms }
    }
}
