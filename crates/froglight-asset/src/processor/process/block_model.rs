use std::any::TypeId;

use bevy_app::{App, Update};
use bevy_asset::Assets;
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
};
use bevy_state::state::OnEnter;
use bevy_transform::components::Transform;
use bevy_utils::HashMap;
use froglight_common::Direction;

use super::resource_atlas::ResourceAtlasState;
use crate::{
    assets::{
        processed::{
            block_model::{BlockDataStorage, BlockModelData, BlockModelStorage},
            BlockModel, ModelTransformIndex,
        },
        unprocessed::{block_definition::DefinitionElement, BlockModelDefinition},
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
                        let uvs = if let Some([x1, y1, x2, y2]) = element_face.uv {
                            match direction {
                                Direction::Up | Direction::Down => {
                                    [[x1, y2], [x2, y2], [x2, y1], [x1, y1]]
                                }
                                Direction::North | Direction::South => {
                                    [[x1, y1], [x2, y1], [x2, y2], [x1, y2]]
                                }
                                Direction::West | Direction::East => {
                                    [[x1, y1], [x1, y2], [x2, y2], [x2, y1]]
                                }
                            }
                            .map(|[x, y]| [x / 16f32, y / 16f32])
                        } else {
                            element.uvs_from(*direction)
                        };

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
            }

            #[cfg(debug_assertions)]
            bevy_log::debug!("BlockModel: Generated \"{key}\"");

            // Insert the `BlockModel` Mesh into the asset storage
            let mesh = meshes.add(model);
            // Inser the `BlockModel` into the `AssetCatalog` queue
            catalog_mesh_queue.push((key.clone(), mesh.id()));

            // Insert the `BlockModel` into the asset storage
            let model = models.add(BlockModel { mesh: mesh.clone(), transforms });
            // Add the `BlockModel` to the `AssetCatalog` queue
            catalog_block_queue.push((key.clone(), model.id()));

            // Insert the `BlockModelData` into the `BlockDataStorage`
            storage.model_data.insert(
                key.clone(),
                BlockModelData { ambient_occlusion: true, asset_id: model.id(), faces },
            );

            // Insert the `BlockModel` and `Mesh` handles into the `BlockModelStorage`
            handles.models.push(model);
            handles.meshes.push(mesh);
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

    /// Inserts the mesh data into the [`Mesh`].
    fn insert_mesh_data(
        positions: [[f32; 3]; 4],
        uvs: [[f32; 2]; 4],
        mesh: &mut Mesh,
        direction: Direction,
    ) {
        if let Some(Indices::U32(indices)) = mesh.indices_mut() {
            let last_index = u32::try_from(indices.len()).unwrap_or_default();
            indices.extend_from_slice(&[
                last_index,
                last_index + 1,
                last_index + 2,
                last_index,
                last_index + 2,
                last_index + 3,
            ]);
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
            mesh_normals.extend([normal, normal, normal, normal]);
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

    /// Resets the [`BlockModelState`].
    fn reset(mut res: ResMut<Self>) {
        res.resource_index = 0;
        res.model_index = 0;
        res.finished = false;
    }
}
