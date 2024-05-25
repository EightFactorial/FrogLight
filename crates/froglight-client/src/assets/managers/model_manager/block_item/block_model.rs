use bevy::prelude::*;
use froglight_assets::assets::{
    model::{DisplayPosition, ModelDisplayTransform, ModelFace},
    BlockModelDefinition, ModelDefinition,
};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

use crate::assets::model_manager::ResolvedModelElement;

/// An Item Model
#[derive(Debug, Reflect)]
pub struct BlockModel {
    /// Whether to enable ambient occlusion
    pub ambient_occlusion: bool,

    /// Model transforms
    ///
    /// Indexed via
    /// [`DisplayPosition`](froglight_assets::assets::model::DisplayPosition).
    pub model_transforms: [ModelDisplayTransform; 7],

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
    pub block_faces: [BlockFaceMesh; 6],
}

impl BlockModel {
    /// Resolves a [`BlockModelDefinition`] into a [`BlockModel`].
    #[must_use]
    #[allow(unused_variables)]
    pub fn resolve_definition(
        key: &ResourceKey,
        def: &BlockModelDefinition,
        definitions: &HashMap<ResourceKey, ModelDefinition>,
        _mesh_assets: &mut Assets<Mesh>,
    ) -> Self {
        // Get the ambient occlusion
        let ambient_occlusion =
            Self::recursive_occlusion(key, def, definitions).unwrap_or_else(|| {
                #[cfg(debug_assertions)]
                warn!("Failed to get ambient occlusion for block model: \"{key}\"");
                BlockModelDefinition::ambient_occlusion_default()
            });

        // Get the model transforms
        let model_transforms = Self::recursive_transforms(key, def, definitions);

        // Get the block elements
        let _elements = Self::recursive_elements(key, def, definitions);

        #[allow(unreachable_code)]
        Self {
            ambient_occlusion,
            model_transforms,
            block_model: todo!("Generate Block Mesh"),
            block_faces: todo!("Generate BlockFaceMeshes"),
        }
    }

    /// Recursively get the ambient occlusion for the model.
    ///
    /// Checks the current model definition, then the parent model definition.
    #[must_use]
    #[allow(unused_variables)]
    fn recursive_occlusion(
        key: &ResourceKey,
        def: &BlockModelDefinition,
        definitions: &HashMap<ResourceKey, ModelDefinition>,
    ) -> Option<bool> {
        // If the ambient occlusion is defined, return it
        if def.ambient_occlusion.is_some() {
            return def.ambient_occlusion;
        }

        // If the parent is defined, check the parent for the ambient occlusion
        if let Some(parent) = &def.parent {
            if let Some(parent_def) = definitions.get(parent) {
                // The parent model must be a block model
                if let ModelDefinition::Block(parent_def) = parent_def {
                    // Recurse into the parent for the ambient occlusion
                    return Self::recursive_occlusion(parent, parent_def, definitions);
                }

                #[cfg(debug_assertions)]
                error!("Parent is not a block model: \"{key}\" -> \"{parent}\"");
            } else {
                #[cfg(debug_assertions)]
                warn!("No parent for block model: \"{key}\" -> \"{parent}\"");
            }
        }

        None
    }

    /// Get all of the model transforms for the model.
    #[must_use]
    fn recursive_transforms(
        key: &ResourceKey,
        def: &BlockModelDefinition,
        definitions: &HashMap<ResourceKey, ModelDefinition>,
    ) -> [ModelDisplayTransform; 7] {
        std::array::from_fn(|position_index| {
            Self::recurse_transform(
                DisplayPosition::from_index(position_index),
                key,
                def,
                definitions,
            )
            .unwrap_or_else(|| {
                #[cfg(debug_assertions)]
                warn!(
                    "Failed to get model transform for block model: \"{key}\" -> {:?}",
                    DisplayPosition::from_index(position_index)
                );
                ModelDisplayTransform::default()
            })
        })
    }

    /// Recursively get the model transform for the given display position.
    ///
    /// Checks the current model definition, then the parent model definition.
    #[must_use]
    #[allow(unused_variables)]
    fn recurse_transform(
        display: DisplayPosition,
        key: &ResourceKey,
        def: &BlockModelDefinition,
        definitions: &HashMap<ResourceKey, ModelDefinition>,
    ) -> Option<ModelDisplayTransform> {
        def.display.as_ref().and_then(|map| map.get(&display)).copied().or_else(|| {
            // If the parent is defined, check the parent for the model transform
            if let Some(parent) = &def.parent {
                if let Some(parent_def) = definitions.get(parent) {
                    if let ModelDefinition::Block(parent_def) = parent_def {
                        // Recurse into the parent for the model transform
                        return Self::recurse_transform(display, parent, parent_def, definitions);
                    }

                    #[cfg(debug_assertions)]
                    error!("Parent is not a block model: \"{key}\" -> \"{parent}\"");
                } else {
                    #[cfg(debug_assertions)]
                    warn!("No parent for block model: \"{key}\" -> \"{parent}\"");
                }
            }

            None
        })
    }

    /// Get the elements for the model.
    fn recursive_elements(
        key: &ResourceKey,
        def: &BlockModelDefinition,
        definitions: &HashMap<ResourceKey, ModelDefinition>,
    ) -> Vec<ResolvedModelElement> {
        let mut textures = HashMap::new();
        if let Some(def_textures) = &def.textures {
            textures.extend(&def_textures.0);
        }
        Self::recurse_elements(key, def, &mut textures, definitions)
    }

    /// Recursively get the elements for the model.
    ///
    /// Checks the current model definition, then the parent model definition.
    fn recurse_elements<'a>(
        key: &ResourceKey,
        def: &BlockModelDefinition,
        textures: &mut HashMap<&'a String, &'a String>,
        definitions: &'a HashMap<ResourceKey, ModelDefinition>,
    ) -> Vec<ResolvedModelElement> {
        if let Some(elements) = &def.elements {
            // Return the elements from this model
            elements
                .iter()
                .filter_map(|element| ResolvedModelElement::resolve_from(key, element, textures))
                .collect()
        } else {
            // If the parent is defined, check the parent for elements
            if let Some(parent) = &def.parent {
                if let Some(parent_def) = definitions.get(parent) {
                    if let ModelDefinition::Block(parent_def) = parent_def {
                        // Add the parent textures to the textures list
                        if let Some(parent_textures) = &parent_def.textures {
                            textures.extend(&parent_textures.0);
                        }

                        // Recurse into the parent for elements
                        return Self::recurse_elements(key, parent_def, textures, definitions);
                    }

                    #[cfg(debug_assertions)]
                    error!("Parent is not a block model: \"{key}\" -> \"{parent}\"");
                } else {
                    #[cfg(debug_assertions)]
                    warn!("No parent for block model: \"{key}\" -> \"{parent}\"");
                }
            }

            Vec::new()
        }
    }

    /// Returns the face mesh for the given face.
    #[must_use]
    pub fn get_face(&self, face: ModelFace) -> &BlockFaceMesh { &self.block_faces[face.as_index()] }
}

/// A mesh for a block face
#[derive(Debug, Reflect)]
pub struct BlockFaceMesh {
    /// The mesh indices
    pub indices: Vec<u32>,
    /// The mesh positions
    pub position: Vec<[f32; 3]>,
    /// The mesh normals
    pub normals: Vec<[f32; 3]>,
    /// The mesh UVs
    pub uvs: Vec<[f32; 2]>,
}

impl BlockFaceMesh {
    /// Returns the default mesh for the given face.
    // TODO: Check if these are correct
    #[must_use]
    pub fn default_for_face(face: ModelFace) -> Self {
        match face {
            ModelFace::Down => Self {
                indices: vec![0, 1, 2, 0, 2, 3],
                position: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
                normals: vec![[0.0, -1.0, 0.0]; 4],
                uvs: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            },
            ModelFace::Up => Self {
                indices: vec![0, 1, 2, 0, 2, 3],
                position: vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
                normals: vec![[0.0, 1.0, 0.0]; 4],
                uvs: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            },
            ModelFace::North => Self {
                indices: vec![0, 1, 2, 0, 2, 3],
                position: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
                normals: vec![[0.0, 0.0, -1.0]; 4],
                uvs: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            },
            ModelFace::South => Self {
                indices: vec![0, 1, 2, 0, 2, 3],
                position: vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
                normals: vec![[0.0, 0.0, 1.0]; 4],
                uvs: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            },
            ModelFace::West => Self {
                indices: vec![0, 1, 2, 0, 2, 3],
                position: vec![[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0]],
                normals: vec![[-1.0, 0.0, 0.0]; 4],
                uvs: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            },
            ModelFace::East => Self {
                indices: vec![0, 1, 2, 0, 2, 3],
                position: vec![[1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]],
                normals: vec![[1.0, 0.0, 0.0]; 4],
                uvs: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            },
        }
    }
}
