use std::any::TypeId;

use bevy_asset::{AssetId, Assets};
use bevy_log::error;
use bevy_math::prelude::Cuboid;
use bevy_render::{
    mesh::{Indices, Mesh, MeshBuilder, Meshable, PrimitiveTopology, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    texture::Image,
};
use bevy_transform::components::Transform;
use bevy_utils::HashMap;
use froglight_common::Direction;
use glam::{FloatExt, Quat, Vec3};

use super::BlockModelState;
use crate::{
    assets::{
        processed::{
            block_model::{
                BlockDataStorage, BlockDataStorageInner, BlockModelData, BlockModelStorage,
            },
            BlockModel, ModelFaceData, ModelTransformIndex,
        },
        unprocessed::{
            block_definition::{DefinitionElement, ElementFace},
            BlockModelDefinition,
        },
    },
    AssetCatalog,
};

impl BlockModelState {
    pub(super) fn generate_block_models(
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
            // Get the `BlockModelDefinition`
            let Some(definition) = definitions.get(definition_id) else {
                error!("BlockModelDefinition not found: \"{key}\"");
                continue;
            };

            // Generate the `BlockModel` and `BlockModelData`
            if let Some((model, transforms, faces)) =
                Self::generate_block_model(catalog, storage, definition, definitions)
            {
                // Insert the `BlockModel` Mesh into the asset storage
                let mesh = meshes.add(model);
                // Insert the `BlockModel` into the `AssetCatalog` queue
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

    fn generate_block_model(
        catalog: &AssetCatalog,
        storage: &mut BlockDataStorageInner,
        definition: &BlockModelDefinition,
        definitions: &Assets<BlockModelDefinition>,
    ) -> Option<(Mesh, [Transform; 8], [ModelFaceData; 6])> {
        // Get the model transforms
        let mut transforms = [Transform::default(); 8];
        for (index, transform) in
            transforms.iter_mut().enumerate().map(|(i, v)| (ModelTransformIndex::from(i), v))
        {
            Self::recurse_for_transform(index, transform, catalog, definition, definitions);
        }

        // Create the BlockModel Mesh
        let mut model = mesh_default();

        // Create a BlockFaceData for each face
        let mut faces: [ModelFaceData; 6] = [
            ModelFaceData::default(),
            ModelFaceData::default(),
            ModelFaceData::default(),
            ModelFaceData::default(),
            ModelFaceData::default(),
            ModelFaceData::default(),
        ];

        if let Some(elements) = Self::recurse_for_elements(catalog, definition, definitions) {
            for element in elements {
                Self::generate_face_model(
                    catalog,
                    storage,
                    definition,
                    definitions,
                    element,
                    &mut model,
                    &mut faces,
                );
            }
        } else {
            return None;
        }

        Some((model, transforms, faces))
    }

    fn generate_face_model(
        catalog: &AssetCatalog,
        storage: &mut BlockDataStorageInner,
        definition: &BlockModelDefinition,
        definitions: &Assets<BlockModelDefinition>,
        element: &DefinitionElement,
        model: &mut Mesh,
        faces: &mut [ModelFaceData; 6],
    ) {
        let from = Vec3::from(element.from) / Vec3::splat(16f32);
        let to = Vec3::from(element.to) / Vec3::splat(16f32);

        // Create a Cuboid Mesh from the element's corners
        let mut cuboid = Cuboid::from_corners(from, to).mesh().build();

        // Rotate the cuboid based on the element's rotation
        if let Some(rotation) = element.rotation {
            // I'm pretty sure I need this...
            let _origin = Vec3::from(rotation.origin).normalize_or_zero();

            let quat = Quat::from_axis_angle(
                match rotation.axis {
                    'x' => Vec3::X,
                    'y' => Vec3::Y,
                    'z' => Vec3::Z,
                    _ => unreachable!(),
                },
                rotation.angle.to_radians(),
            );

            // Rotate the cuboid's vertices
            cuboid.rotate_by(quat);
        }

        // Offset the cuboid's vertices based on the model's position
        cuboid.translate_by(from.midpoint(to) - Vec3::splat(0.5));

        // Append face-specific data to the cuboid mesh and faces array
        for (dir_index, direction) in (0..6).map(Direction::from).enumerate() {
            // Get the data indexes
            let indexes: [usize; 4] = [0, 1, 2, 3].map(|i| i + (dir_index * 4));

            if let Some(element_face) = element.faces.get(&direction) {
                // Skip blocks with missing textures
                let Some(texture_asset_id) =
                    Self::get_texture(catalog, element_face, definition, definitions)
                else {
                    return;
                };

                // Get the face
                let face = &mut faces[usize::from(direction)];

                // Copy the mesh indices to the face
                Self::copy_indices(face, &cuboid, direction);

                // Copy mesh vertices to the face
                Self::copy_vertices(face, &mut cuboid, indexes);

                // Copy mesh normals
                Self::copy_normals(face, &cuboid, indexes);

                // Copy mesh uvs to the face and create atlas uvs for both
                Self::copy_and_generate_uvs(
                    texture_asset_id,
                    element_face,
                    indexes,
                    direction,
                    face,
                    &mut cuboid,
                    storage,
                );
            } else {
                // Set the fallback UVs
                Self::fallback_uvs(&mut cuboid, indexes, direction);
            }
        }

        // Append the cuboid mesh to the model mesh
        model.merge(&cuboid);
    }

    /// Copy the cuboid's indices to the face
    fn copy_indices(face: &mut ModelFaceData, cuboid: &Mesh, direction: Direction) {
        let (Indices::U32(cube_ind), Indices::U32(face_ind)) =
            (cuboid.indices().unwrap(), &mut face.indices)
        else {
            unreachable!();
        };

        face_ind.extend(
            &cube_ind[match direction {
                Direction::Up => 0..6,
                Direction::Down => 6..12,
                Direction::North => 12..18,
                Direction::South => 18..24,
                Direction::East => 24..30,
                Direction::West => 30..36usize,
            }],
        );
    }

    /// Copy the cuboid's vertices to the face
    ///
    /// Also offsets the vertices based on the model's position
    fn copy_vertices(face: &mut ModelFaceData, cuboid: &mut Mesh, indexes: [usize; 4]) {
        let Some(VertexAttributeValues::Float32x3(cube_pos)) =
            cuboid.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        else {
            unreachable!();
        };

        let mut face_vertices = Vec::new();
        for indice in &indexes {
            face_vertices.push(cube_pos[*indice]);
        }

        face.append_to_face(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(face_vertices),
        );
    }

    /// Copy the cuboid's normals to the face
    fn copy_normals(face: &mut ModelFaceData, cuboid: &Mesh, indexes: [usize; 4]) {
        let Some(VertexAttributeValues::Float32x3(cube_norm)) =
            cuboid.attribute(Mesh::ATTRIBUTE_NORMAL)
        else {
            unreachable!();
        };

        let mut face_normals = Vec::new();
        for index in indexes {
            face_normals.push(cube_norm[index]);
        }

        face.append_to_face(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float32x3(face_normals));
    }

    /// Copy the face's uvs and generate the atlas uvs
    fn copy_and_generate_uvs(
        texture_asset_id: AssetId<Image>,
        element_face: &ElementFace,
        indexes: [usize; 4],
        direction: Direction,
        face: &mut ModelFaceData,
        cuboid: &mut Mesh,
        storage: &BlockDataStorageInner,
    ) {
        let mut face_uvs = Vec::new();
        if let Some(element_uvs) = element_face.uvs_from() {
            face_uvs = element_uvs.to_vec();
        } else if let Some(VertexAttributeValues::Float32x2(cube_uv)) =
            cuboid.attribute(Mesh::ATTRIBUTE_UV_0)
        {
            for index in indexes {
                face_uvs.push(cube_uv[index]);
            }

            match direction {
                Direction::North => {
                    face_uvs.rotate_right(2);
                }
                Direction::South => {
                    face_uvs.rotate_right(3);
                }
                Direction::East | Direction::Up => {
                    face_uvs.swap(0, 1);
                    face_uvs.swap(2, 3);
                }
                _ => {}
            }
        } else {
            unreachable!()
        }

        // Get the atlas UVs
        let mut atlas_uvs = Vec::new();
        if let Some(texture_index) = storage.block_atlas.get_texture_index(texture_asset_id) {
            // Get the texture's size and rect
            let texture_size = storage.block_atlas.size.as_vec2();
            let texture_rect = storage.block_atlas.textures[texture_index].as_rect();

            // Remap the face's uvs to the texture's rect
            for [u, v] in &face_uvs {
                atlas_uvs.push([
                    u.remap(
                        1.0,
                        0.0,
                        texture_rect.min.x / texture_size.x,
                        texture_rect.max.x / texture_size.x,
                    ),
                    v.remap(
                        0.0,
                        1.0,
                        texture_rect.min.y / texture_size.y,
                        texture_rect.max.y / texture_size.y,
                    ),
                ]);
            }
        } else {
            // Copy the face's uvs to the atlas uvs
            atlas_uvs.extend_from_slice(&face_uvs);
        }

        // Append the atlas uvs to the cuboid mesh
        if let Some(VertexAttributeValues::Float32x2(cuboid_atlas)) =
            cuboid.attribute_mut(Mesh::ATTRIBUTE_UV_1)
        {
            cuboid_atlas.extend_from_slice(&atlas_uvs);
        } else {
            cuboid.insert_attribute(
                Mesh::ATTRIBUTE_UV_1,
                VertexAttributeValues::Float32x2(atlas_uvs.clone()),
            );
        }

        // Append the uvs and atlas uvs to the face
        face.append_to_face(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(face_uvs));
        face.append_to_face(Mesh::ATTRIBUTE_UV_1, VertexAttributeValues::Float32x2(atlas_uvs));
    }

    // Copy the cuboid uvs as the atlas uvs
    fn fallback_uvs(cuboid: &mut Mesh, indexes: [usize; 4], direction: Direction) {
        let Some(VertexAttributeValues::Float32x2(cube_uv)) =
            cuboid.attribute(Mesh::ATTRIBUTE_UV_0)
        else {
            unreachable!();
        };

        let mut atlas_uvs = Vec::new();
        for index in indexes {
            atlas_uvs.push(cube_uv[index]);
        }

        match direction {
            Direction::South | Direction::East | Direction::West => {
                atlas_uvs.rotate_right(1);
            }
            _ => {}
        }

        if let Some(VertexAttributeValues::Float32x2(cuboid_atlas)) =
            cuboid.attribute_mut(Mesh::ATTRIBUTE_UV_1)
        {
            cuboid_atlas.extend_from_slice(&atlas_uvs);
        } else {
            cuboid.insert_attribute(
                Mesh::ATTRIBUTE_UV_1,
                VertexAttributeValues::Float32x2(atlas_uvs.clone()),
            );
        }
    }
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
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float32x3(Vec::new()));
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float32x3(Vec::new()));

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(Vec::new()));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_1, VertexAttributeValues::Float32x2(Vec::new()));

    mesh
}
