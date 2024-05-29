use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology, VertexAttributeValues},
        render_asset::RenderAssetUsages,
    },
};
use froglight_assets::assets::{
    model::{DisplayPosition, ModelDisplayTransform, ModelFace},
    BlockModelDefinition, ModelDefinition,
};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

use super::resolve;
use crate::assets::{
    model_manager::{ResolvedElementFace, ResolvedModelElement},
    AssetManager,
};

/// An Item Model
#[derive(Debug, Reflect)]
pub struct BlockModel {
    /// Whether to enable ambient occlusion
    pub ambient_occlusion: bool,

    /// Block textures
    ///
    /// Both `block_model` and `block_faces` index into this array.
    pub textures: Vec<Handle<Image>>,

    /// Block model mesh
    ///
    /// This is used for rendering the block in the player's hand,
    /// in the inventory, in item frames, etc.
    pub block_model: Handle<Mesh>,

    /// Block face meshes
    ///
    /// This is used for generating terrain meshes.
    ///
    /// Indexed via
    /// [`ModelFace`](froglight_assets::assets::model::ModelFace).
    pub block_faces: [Option<BlockFaceMesh>; 6],

    /// Model transforms
    ///
    /// Indexed via
    /// [`DisplayPosition`](froglight_assets::assets::model::DisplayPosition).
    pub model_transforms: [ModelDisplayTransform; 7],
}

impl BlockModel {
    /// Resolves a [`BlockModelDefinition`] into a [`BlockModel`].
    #[must_use]
    #[allow(clippy::too_many_lines, clippy::missing_panics_doc)]
    pub fn resolve_definition(
        key: &ResourceKey,
        def: &BlockModelDefinition,
        definitions: &HashMap<ResourceKey, ModelDefinition>,
        asset_manager: &AssetManager,
        mesh_assets: &mut Assets<Mesh>,
    ) -> Self {
        // Get the ambient occlusion
        let ambient_occlusion = resolve::recursive_occlusion(key, def, definitions)
            .unwrap_or_else(BlockModelDefinition::ambient_occlusion_default);

        // Get the model transforms
        let model_transforms = resolve::recursive_transforms(key, def, definitions);

        let mut block_faces: [Option<BlockFaceMesh>; 6] = [None, None, None, None, None, None];
        let mut textures: Vec<ResourceKey> = Vec::new();

        // Get the elements for the model
        for element in resolve::recursive_elements(key, def, definitions) {
            for element_face in element.faces.iter().flatten() {
                // Get the block face
                let face_index = element_face.cullface.as_index();
                let block_face = block_faces[face_index].get_or_insert(BlockFaceMesh::default());

                // Add the element face data to the block face
                block_face.add_element_data(element_face, &element, &mut textures);
            }
        }

        // Combine all block faces into a block model
        let mut block_model = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
        for face in block_faces.iter().flatten() {
            face.add_to_mesh(&mut block_model);
        }

        Self {
            ambient_occlusion,
            model_transforms,
            block_faces,
            // Add the block model asset
            block_model: mesh_assets.add(block_model),
            // Resolve the texture assets
            textures: textures
                .into_iter()
                .map(|key| {
                    asset_manager.textures.get(&key).unwrap_or_else(|| {
                        #[cfg(debug_assertions)]
                        error!("Failed to get texture for block model: \"{key}\"");
                        &asset_manager.textures[&AssetManager::FALLBACK_TEXTURE]
                    })
                })
                .cloned()
                .collect(),
        }
    }

    /// Returns the [`BlockFaceMesh`] for the given [`ModelFace`].
    #[must_use]
    pub fn get_face(&self, face: ModelFace) -> Option<&BlockFaceMesh> {
        self.block_faces[face.as_index()].as_ref()
    }

    /// Returns the [`ModelDisplayTransform`] for the given [`DisplayPosition`].
    #[must_use]
    pub fn get_transform(&self, display: DisplayPosition) -> &ModelDisplayTransform {
        &self.model_transforms[display.as_index()]
    }
}

/// A mesh for a block face
#[derive(Debug, Default, Clone, Reflect)]
pub struct BlockFaceMesh {
    /// The mesh indices
    pub indices: Vec<u32>,
    /// The mesh positions
    pub position: Vec<[f32; 3]>,
    /// The mesh normals
    pub normals: Vec<[f32; 3]>,
    /// The mesh UVs
    pub uvs: Vec<[f32; 2]>,
    /// The mesh texture indices
    ///
    /// This is an index into the [`BlockModel::textures`] array.
    pub textures: Vec<u32>,
}

impl BlockFaceMesh {
    /// Adds an [`ResolvedElementFace`] to the [`BlockFaceMesh`].
    pub fn add_element_data(
        &mut self,
        element_face: &ResolvedElementFace,
        element: &ResolvedModelElement,
        textures: &mut Vec<ResourceKey>,
    ) {
        // Add the face indices
        {
            // Create the face indices
            #[allow(clippy::cast_possible_truncation)]
            let mut face_indices = {
                let start = self.position.len() as u32;
                [start, start + 2, start + 1, start, start + 3, start + 2]
            };

            // Rotate the indices if the face texture is rotated
            #[allow(clippy::cast_sign_loss)]
            face_indices.rotate_right(element_face.rotation.min(270) as usize / 90);

            self.indices.extend(face_indices);
        }

        // Add the face positions
        //
        // Rotate quads to point out of the block
        {
            match element_face.cullface {
                ModelFace::Down => {
                    self.position.extend(&[
                        [element.from[0] / 16.0, element.from[1] / 16.0, element.to[2] / 16.0],
                        [element.to[0] / 16.0, element.from[1] / 16.0, element.to[2] / 16.0],
                        [element.to[0] / 16.0, element.from[1] / 16.0, element.from[2] / 16.0],
                        [element.from[0] / 16.0, element.from[1] / 16.0, element.from[2] / 16.0],
                    ]);
                }
                ModelFace::Up => {
                    self.position.extend(&[
                        [element.from[0] / 16.0, element.to[1] / 16.0, element.from[2] / 16.0],
                        [element.to[0] / 16.0, element.to[1] / 16.0, element.from[2] / 16.0],
                        [element.to[0] / 16.0, element.to[1] / 16.0, element.to[2] / 16.0],
                        [element.from[0] / 16.0, element.to[1] / 16.0, element.to[2] / 16.0],
                    ]);
                }
                ModelFace::North => {
                    self.position.extend(&[
                        [element.to[0] / 16.0, element.to[1] / 16.0, element.from[2] / 16.0],
                        [element.from[0] / 16.0, element.to[1] / 16.0, element.from[2] / 16.0],
                        [element.from[0] / 16.0, element.from[1] / 16.0, element.from[2] / 16.0],
                        [element.to[0] / 16.0, element.from[1] / 16.0, element.from[2] / 16.0],
                    ]);
                }
                ModelFace::South => {
                    self.position.extend(&[
                        [element.from[0] / 16.0, element.to[1] / 16.0, element.to[2] / 16.0],
                        [element.to[0] / 16.0, element.to[1] / 16.0, element.to[2] / 16.0],
                        [element.to[0] / 16.0, element.from[1] / 16.0, element.to[2] / 16.0],
                        [element.from[0] / 16.0, element.from[1] / 16.0, element.to[2] / 16.0],
                    ]);
                }
                ModelFace::West => {
                    self.position.extend(&[
                        [element.from[0] / 16.0, element.to[1] / 16.0, element.from[2] / 16.0],
                        [element.from[0] / 16.0, element.to[1] / 16.0, element.to[2] / 16.0],
                        [element.from[0] / 16.0, element.from[1] / 16.0, element.to[2] / 16.0],
                        [element.from[0] / 16.0, element.from[1] / 16.0, element.from[2] / 16.0],
                    ]);
                }
                ModelFace::East => {
                    self.position.extend(&[
                        [element.to[0] / 16.0, element.to[1] / 16.0, element.to[2] / 16.0],
                        [element.to[0] / 16.0, element.to[1] / 16.0, element.from[2] / 16.0],
                        [element.to[0] / 16.0, element.from[1] / 16.0, element.from[2] / 16.0],
                        [element.to[0] / 16.0, element.from[1] / 16.0, element.to[2] / 16.0],
                    ]);
                }
            }
        }

        // Create the face normal from the `from` and `to` coordinates
        //
        // TODO: Check if this is correct
        {
            // Get the cross product of the `from` and `to` coordinates
            let mut cross = Vec3::from_array(element.from)
                .cross(Vec3::from_array(element.to))
                .normalize_or_zero();

            // Rotate the cross product if the element has a rotation
            //
            // TODO: Respect `rotation.origin` and `rotation.rescale`
            if let Some(rotation) = element.rotation {
                let quat =
                    Quat::from_axis_angle(rotation.axis.as_identity().into(), rotation.angle);
                cross = quat.mul_vec3(cross);
            }
            self.normals.extend(&[cross.into(); 4]);
        }

        // Add the face uvs
        self.uvs.extend(&[
            [element_face.uv[0] / 16.0, element_face.uv[1] / 16.0],
            [element_face.uv[2] / 16.0, element_face.uv[1] / 16.0],
            [element_face.uv[2] / 16.0, element_face.uv[3] / 16.0],
            [element_face.uv[0] / 16.0, element_face.uv[3] / 16.0],
        ]);

        // Add the face texture
        #[allow(clippy::cast_possible_truncation)]
        {
            let texture_index = textures
                .iter()
                .position(|texture| texture == &element_face.texture)
                .unwrap_or_else(|| {
                    textures.push(element_face.texture.clone());
                    textures.len() - 1
                });
            self.textures.extend([texture_index as u32; 4]);
        }
    }

    /// Adds the [`BlockFaceMesh`] to a [`Mesh`].
    pub fn add_to_mesh(&self, mesh: &mut Mesh) {
        // Get the index offset
        let offset = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .and_then(|p| u32::try_from(p.len()).ok())
            .unwrap_or_default();

        // Insert or extend the attributes
        if let Some(Indices::U32(indices)) = mesh.indices_mut() {
            indices.extend(self.indices.iter().map(|index| *index + offset));
        } else {
            mesh.insert_indices(Indices::U32(self.indices.clone()));
        }

        if let Some(VertexAttributeValues::Float32x3(position)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            position.extend(&self.position);
        } else {
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.position.clone());
        }

        if let Some(VertexAttributeValues::Float32x3(normals)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL)
        {
            normals.extend(&self.normals);
        } else {
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone());
        }

        if let Some(VertexAttributeValues::Float32x2(uvs)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0)
        {
            uvs.extend(&self.uvs);
        } else {
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());
        }

        // if let Some(textures) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_1) {
        //     textures.extend(self.textures.iter().cloned());
        // } else {
        //     mesh.insert_attribute(Mesh::ATTRIBUTE_UV_1,
        // self.textures.clone()); }
    }
}

impl From<BlockFaceMesh> for Mesh {
    fn from(value: BlockFaceMesh) -> Self { Mesh::from(&value) }
}
impl From<&BlockFaceMesh> for Mesh {
    fn from(value: &BlockFaceMesh) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
        value.add_to_mesh(&mut mesh);
        mesh
    }
}
