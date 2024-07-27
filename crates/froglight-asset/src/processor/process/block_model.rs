use std::any::TypeId;

use bevy_app::{App, Update};
use bevy_asset::{AssetId, Assets};
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_log::error;
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::{
    mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    texture::Image,
};
use bevy_state::state::OnEnter;
use bevy_transform::components::Transform;
use bevy_utils::HashMap;
use froglight_common::Direction;
use glam::FloatExt;

use super::resource_atlas::ResourceAtlasState;
use crate::{
    assets::{
        processed::{
            block_model::{BlockDataStorage, BlockModelData, BlockModelStorage},
            BlockModel, ModelTransformIndex,
        },
        unprocessed::{
            block_definition::{DefinitionElement, ResourceOrVariable},
            BlockModelDefinition,
        },
    },
    AssetCatalog, AssetLoadState, ResourcePack, ResourcePackList,
};

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

    #[allow(clippy::too_many_lines)]
    fn generate_block_models(
        definitions: &Assets<BlockModelDefinition>,
        models: &mut Assets<BlockModel>,
        meshes: &mut Assets<Mesh>,

        catalog: &mut AssetCatalog,
        storage: &mut BlockDataStorage,
        handles: &mut BlockModelStorage,
    ) {
        let untyped_definitions = catalog.inner.get(&TypeId::of::<BlockModelDefinition>()).unwrap();
        let storage = &mut *storage.write();

        let mut catalog_mesh_queue = Vec::with_capacity(untyped_definitions.len());
        let mut catalog_block_queue = Vec::with_capacity(untyped_definitions.len());

        for (key, definition_id) in untyped_definitions
            .iter()
            .map(|(k, v)| (k, v.typed_debug_checked::<BlockModelDefinition>()))
        {
            let Some(definition) = definitions.get(definition_id) else {
                error!("BlockModelDefinition not found: \"{key}\"");
                continue;
            };

            // Get the model transforms
            let mut transforms = [Transform::default(); 8];
            for (index, transform) in
                transforms.iter_mut().enumerate().map(|(i, v)| (ModelTransformIndex::from(i), v))
            {
                Self::recurse_for_transform(index, transform, catalog, definition, definitions);
            }

            // Create the BlockModel Mesh
            let mut model = Self::mesh_default();

            // Create the BlockModelData face meshes
            let mut faces: [Mesh; 6] = [
                model.clone(),
                model.clone(),
                model.clone(),
                model.clone(),
                model.clone(),
                model.clone(),
            ];

            if let Some(elements) = Self::recurse_for_elements(catalog, definition, definitions) {
                for element in elements {
                    for (direction, element_face) in &element.faces {
                        let positions = element.positions_from(*direction);
                        let mut uvs = element_face
                            .uvs_from()
                            .unwrap_or(element.uvs_from(*direction, element_face.rotation));

                        if let Some(texture_index) = match &element_face.texture {
                            ResourceOrVariable::Resource(key) => catalog.get::<Image>(key),
                            ResourceOrVariable::Variable(var) => Self::recurse_for_resource(
                                catalog,
                                var.to_owned(),
                                definition,
                                definitions,
                                &mut HashMap::default(),
                            ),
                        }
                        .and_then(|id| storage.block_atlas.get_texture_index(id))
                        {
                            let texture_size = storage.block_atlas.size.as_vec2();
                            let texture_rect =
                                storage.block_atlas.textures[texture_index].as_rect();

                            for uv in &mut uvs {
                                uv[0] = uv[0].remap(
                                    0.0,
                                    1.0,
                                    texture_rect.min.x / texture_size.x,
                                    texture_rect.max.x / texture_size.x,
                                );
                                uv[1] = uv[1].remap(
                                    0.0,
                                    1.0,
                                    texture_rect.min.y / texture_size.y,
                                    texture_rect.max.y / texture_size.y,
                                );
                            }
                        } else {
                            #[cfg(debug_assertions)]
                            bevy_log::warn!(
                                "BlockModel: \"{key}\" has no texture for {direction}: \"{:?}\"",
                                element_face.texture
                            );
                            continue;
                        }

                        // Add the element to the face
                        Self::insert_mesh_data(
                            positions,
                            uvs,
                            &mut faces[usize::from(*direction)],
                            *direction,
                        );
                        // Add the element to the model
                        Self::insert_mesh_data(positions, uvs, &mut model, *direction);
                    }
                }
            } else {
                #[cfg(debug_assertions)]
                bevy_log::warn!("BlockModel: \"{key}\" has no elements");
                continue;
            }

            #[cfg(debug_assertions)]
            bevy_log::trace!("BlockModel: Generated \"{key}\"");

            // Insert the `BlockModel` Mesh into the asset storage
            let mesh = meshes.add(model);
            // Inser the `BlockModel` into the `AssetCatalog` queue
            catalog_mesh_queue.push((key.clone(), mesh.id()));

            // Insert the `BlockModel` into the asset storage
            let model = models.add(BlockModel { mesh, transforms });
            // Add the `BlockModel` to the `AssetCatalog` queue
            catalog_block_queue.push((key.clone(), model.id()));

            // Insert the `BlockModelData` into the `BlockDataStorage`
            storage.model_data.insert(
                key.clone(),
                BlockModelData {
                    ambient_occlusion: definition
                        .ambient_occlusion
                        .unwrap_or(BlockModelDefinition::DEFAULT_AMBIENT_OCCLUSION),
                    asset_id: model.id(),
                    faces,
                },
            );

            // Insert the `BlockModel` handle into the `BlockModelStorage`
            handles.push(model);
        }

        // Add the queued `BlockModel`s to the `AssetCatalog`
        {
            let untyped_models = catalog
                .inner
                .entry(TypeId::of::<BlockModel>())
                .or_insert_with(|| HashMap::with_capacity(catalog_block_queue.len()).into());
            for (key, id) in catalog_block_queue {
                untyped_models.insert(key, id.untyped());
            }
        }

        // Add the queued `Mesh`es to the `AssetCatalog`
        {
            let untyped_meshes = catalog
                .inner
                .entry(TypeId::of::<Mesh>())
                .or_insert_with(|| HashMap::with_capacity(catalog_mesh_queue.len()).into());
            for (key, id) in catalog_mesh_queue {
                untyped_meshes.insert(key, id.untyped());
            }
        }

        // Remove all `BlockModelDefinition`s from the `AssetCatalog`
        catalog.inner.remove(&TypeId::of::<BlockModelDefinition>());

        #[cfg(debug_assertions)]
        bevy_log::info!("AssetCatalog: Finished Generating BlockModels");
    }

    /// Returns a default [`Mesh`] for a [`BlockModel`].
    ///
    /// The default mesh is a [`PrimitiveTopology::TriangleList`] with no
    /// vertices.
    ///
    /// Has the following attributes:
    /// - [`Mesh::ATTRIBUTE_POSITION`] : [`VertexAttributeValues::Float32x3`]
    /// - [`Mesh::ATTRIBUTE_NORMAL`] : [`VertexAttributeValues::Float32x3`]
    /// - [`Mesh::ATTRIBUTE_UV_0`] : [`VertexAttributeValues::Float32x2`]
    fn mesh_default() -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
        mesh.insert_indices(Indices::U32(Vec::new()));
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(Vec::new()),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float32x3(Vec::new()));
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(Vec::new()));

        mesh
    }

    // Indices::U32(vec![
    //     0, 1, 2, 2, 3, 0, // front
    //     4, 5, 6, 6, 7, 4, // back
    //     8, 9, 10, 10, 11, 8, // right
    //     12, 13, 14, 14, 15, 12, // left
    //     16, 17, 18, 18, 19, 16, // top
    //     20, 21, 22, 22, 23, 20, // bottom
    // ]);

    /// Inserts the mesh data into the [`Mesh`].
    fn insert_mesh_data(
        positions: [[f32; 3]; 4],
        uvs: [[f32; 2]; 4],
        mesh: &mut Mesh,
        direction: Direction,
    ) {
        if let Some(Indices::U32(indices)) = mesh.indices_mut() {
            let index =
                indices.get(indices.len().saturating_sub(2)).map(|v| v + 1).unwrap_or_default();
            indices.extend_from_slice(&[index, index + 1, index + 2, index + 2, index + 3, index]);
        }

        if let Some(VertexAttributeValues::Float32x3(mesh_positions)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            mesh_positions.extend(positions);
        }

        #[allow(clippy::cast_precision_loss)]
        if let Some(VertexAttributeValues::Float32x3(mesh_normals)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL)
        {
            let normal = direction.to_axis().to_array().map(|v| v as f32);
            mesh_normals.extend([normal; 4]);
        }

        if let Some(VertexAttributeValues::Float32x2(mesh_uvs)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0)
        {
            mesh_uvs.extend(uvs);
        }
    }

    fn recurse_for_transform(
        index: ModelTransformIndex,
        transform: &mut Transform,

        catalog: &AssetCatalog,
        definition: &BlockModelDefinition,
        definitions: &Assets<BlockModelDefinition>,
    ) {
        if let Some(def_transform) = definition.display.get(&index) {
            *transform = Transform::from(*def_transform);
        } else if let Some(parent_key) = &definition.parent {
            if let Some(parent_id) = catalog.get::<BlockModelDefinition>(parent_key) {
                if let Some(parent_def) = definitions.get(parent_id) {
                    Self::recurse_for_transform(index, transform, catalog, parent_def, definitions);
                }
            }
        }
    }

    fn recurse_for_elements<'a>(
        catalog: &AssetCatalog,
        definition: &'a BlockModelDefinition,
        definitions: &'a Assets<BlockModelDefinition>,
    ) -> Option<&'a [DefinitionElement]> {
        if let Some(elements) = definition.elements.as_ref() {
            return Some(elements);
        } else if let Some(parent_key) = &definition.parent {
            if let Some(parent_id) = catalog.get::<BlockModelDefinition>(parent_key) {
                if let Some(parent_def) = definitions.get(parent_id) {
                    return Self::recurse_for_elements(catalog, parent_def, definitions);
                }
            }
        }
        None
    }

    fn recurse_for_resource<'a>(
        catalog: &'a AssetCatalog,
        mut resource: String,
        definition: &'a BlockModelDefinition,
        definitions: &'a Assets<BlockModelDefinition>,
        variables: &mut HashMap<String, ResourceOrVariable>,
    ) -> Option<AssetId<Image>> {
        variables.extend(definition.textures.clone());

        if let Some(mut variable) = variables.get(&resource) {
            while let ResourceOrVariable::Variable(var) = variable {
                if let Some(value) = variables.get(var) {
                    if variable == value {
                        break;
                    }
                    variable = value;
                } else {
                    resource = var.to_string();
                    break;
                }
            }
            if let ResourceOrVariable::Resource(key) = variable {
                if let Some(asset_id) = catalog.get::<Image>(key) {
                    return Some(asset_id);
                }
                resource = key.to_string();
            }
        }

        if let Some(parent_key) = &definition.parent {
            if let Some(parent_id) = catalog.get::<BlockModelDefinition>(parent_key) {
                if let Some(parent_def) = definitions.get(parent_id) {
                    return Self::recurse_for_resource(
                        catalog,
                        resource,
                        parent_def,
                        definitions,
                        variables,
                    );
                }
            }
        }

        None
    }

    /// Resets the [`BlockModelState`].
    fn reset(mut res: ResMut<Self>) {
        res.resource_index = 0;
        res.model_index = 0;
        res.finished = false;
    }
}
