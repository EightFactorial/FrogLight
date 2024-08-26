use bevy_asset::{Assets, UntypedHandle};
use bevy_log::error;
use bevy_render::{
    mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    texture::Image,
};
use bevy_utils::HashMap;
use froglight_common::ResourceKey;

use super::BlockModelProcessor;
use crate::{
    assets::{
        processed::model::ModelTransformIndex,
        raw::{
            model::{DefinitionElement, DefinitionTransform, ElementFace, ResourceOrVariable},
            BlockModelDefinition,
        },
    },
    AssetCatalog,
};

/// [`BlockModelProcessor`] methods for retrieving model data.
impl BlockModelProcessor {
    /// Get the texture for an [`ElementFace`].
    #[must_use]
    pub(super) fn get_element_texture<'a>(
        element: &ElementFace,
        definition: &BlockModelDefinition,
        definition_key: &ResourceKey,
        catalog: &'a AssetCatalog,
        definitions: &Assets<BlockModelDefinition>,
    ) -> Option<&'a UntypedHandle> {
        match &element.texture {
            ResourceOrVariable::Resource(resource) => {
                catalog.get_untyped::<Image>(&ResourceKey::try_new(resource).ok()?)
            }
            ResourceOrVariable::Variable(variable) => Self::recurse_for_resource(
                variable.clone(),
                definition,
                definition_key,
                definitions,
                catalog,
                &mut HashMap::new(),
            ),
        }
    }

    /// Get's the [`UntypedHandle`] for a variable in a
    /// [`BlockModelDefinition`].
    #[must_use]
    fn recurse_for_resource<'a>(
        mut variable: String,
        definition: &BlockModelDefinition,
        definition_key: &ResourceKey,
        definitions: &Assets<BlockModelDefinition>,
        catalog: &'a AssetCatalog,
        variables: &mut HashMap<String, ResourceOrVariable>,
    ) -> Option<&'a UntypedHandle> {
        // Append the current definition's variables to the variable map
        if let Some(textures) = definition.textures.as_ref() {
            variables.extend(textures.clone());
        }

        // Resolve the variable to a resource, or as far as possible
        if let Some(mut res_variable) = variables.get(&variable) {
            while let ResourceOrVariable::Variable(var) = res_variable {
                if let Some(value) = variables.get(var) {
                    if res_variable == value {
                        break;
                    }
                    res_variable = value;
                } else {
                    variable = var.to_string();
                    break;
                }
            }
            if let ResourceOrVariable::Resource(key) = res_variable {
                let key = ResourceKey::try_new(key).ok()?;
                if let Some(handle) = catalog.get_untyped::<Image>(&key) {
                    return Some(handle);
                }
                variable = key.to_string();
            }
        }

        if let Some(parent) = definition.parent.as_ref() {
            // Look for the variable in the parent definition
            let parent_key = ResourceKey::try_new(parent).ok()?;
            let parent_handle = catalog.get_untyped::<BlockModelDefinition>(&parent_key)?;
            let parent_def = definitions.get(parent_handle.id().typed_debug_checked())?;
            Self::recurse_for_resource(
                variable,
                parent_def,
                &parent_key,
                definitions,
                catalog,
                variables,
            )
        } else {
            error!("BlockModelProcessor: Failed to resolve variable \"{variable}\" in BlockModel \"{definition_key}\"");
            None
        }
    }

    /// Recurse through the definitions for the first occurrence of an ambient
    /// occlusion value.
    ///
    /// If no ambient occlusion value is found, the default value is returned.
    pub(super) fn get_ambient_occlusion(
        definition: &BlockModelDefinition,
        catalog: &AssetCatalog,
        definitions: &Assets<BlockModelDefinition>,
    ) -> bool {
        if let Some(occ) = definition.ambient_occlusion {
            return occ;
        }

        if let Some(parent) = definition
            .parent
            .as_ref()
            .and_then(|p| ResourceKey::try_new(p.to_owned()).ok())
            .and_then(|k| catalog.get_untyped::<BlockModelDefinition>(&k))
            .and_then(|h| definitions.get(h.id().typed_debug_checked()))
        {
            Self::get_ambient_occlusion(parent, catalog, definitions)
        } else {
            BlockModelDefinition::DEFAULT_AMBIENT_OCCLUSION
        }
    }

    /// Recurse through the definitions for the first occurrence of a display
    /// type.
    ///
    /// Only the first occurrence of a display type is used for the model.
    #[must_use]
    pub(super) fn get_display_type<'a>(
        display: ModelTransformIndex,
        definition: &'a BlockModelDefinition,
        catalog: &AssetCatalog,
        definitions: &'a Assets<BlockModelDefinition>,
    ) -> Option<&'a DefinitionTransform> {
        if let Some(displays) = definition.display.as_ref() {
            if let Some(transform) = displays.get(&display) {
                return Some(transform);
            }
        }

        if let Some(parent) = definition.parent.as_ref() {
            let parent_key = ResourceKey::try_new(parent.to_owned()).ok()?;
            let parent_handle = catalog.get_untyped::<BlockModelDefinition>(&parent_key)?;
            let parent = definitions.get(parent_handle.id().typed_debug_checked())?;
            Self::get_display_type(display, parent, catalog, definitions)
        } else {
            None
        }
    }

    /// Recurse through the definitions for the first set of elements.
    ///
    /// Only the first set of elements is used for the model.
    #[must_use]
    pub(super) fn get_elements<'a>(
        definition: &'a BlockModelDefinition,
        catalog: &AssetCatalog,
        definitions: &'a Assets<BlockModelDefinition>,
    ) -> Option<&'a [DefinitionElement]> {
        if definition.elements.is_some() {
            definition.elements.as_deref()
        } else if let Some(parent) = definition.parent.as_ref() {
            let parent_key = ResourceKey::try_new(parent.to_owned()).ok()?;
            let parent_handle = catalog.get_untyped::<BlockModelDefinition>(&parent_key)?;
            let parent = definitions.get(parent_handle.id().typed_debug_checked())?;
            Self::get_elements(parent, catalog, definitions)
        } else {
            None
        }
    }
}

/// [`BlockModelProcessor`] methods for processing model data.
impl BlockModelProcessor {
    /// Creates an empty [`Mesh`] with the default attributes.
    pub(super) fn default_mesh() -> Mesh {
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

    /// Appends the [`Mesh::ATTRIBUTE_POSITION`] values
    /// from an `element mesh` to a `direction mesh`.
    pub(super) fn append_element_positions(
        attribute_group: usize,
        direction_mesh: &mut Mesh,
        element_mesh: &Mesh,
    ) {
        let position_range = (attribute_group * 4)..(attribute_group * 4 + 4);
        match (
            element_mesh.attribute(Mesh::ATTRIBUTE_POSITION),
            direction_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION),
        ) {
            (
                Some(VertexAttributeValues::Float32x3(elem_positions)),
                Some(VertexAttributeValues::Float32x3(dir_positions)),
            ) => {
                dir_positions.extend_from_slice(&elem_positions[position_range]);
            }
            (Some(VertexAttributeValues::Float32x3(elem_positions)), None) => {
                direction_mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    elem_positions[position_range].to_vec(),
                );
            }
            _ => unreachable!("Element will always have Float32x3 positions"),
        }
    }

    /// Appends the [`Mesh::ATTRIBUTE_NORMAL`] values
    /// from an `element mesh` to a `direction mesh`.
    pub(super) fn append_element_normals(
        attribute_group: usize,
        direction_mesh: &mut Mesh,
        element_mesh: &Mesh,
    ) {
        let normal_range = (attribute_group * 4)..(attribute_group * 4 + 4);
        match (
            element_mesh.attribute(Mesh::ATTRIBUTE_NORMAL),
            direction_mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL),
        ) {
            (
                Some(VertexAttributeValues::Float32x3(elem_normals)),
                Some(VertexAttributeValues::Float32x3(dir_normals)),
            ) => {
                dir_normals.extend_from_slice(&elem_normals[normal_range]);
            }
            (Some(VertexAttributeValues::Float32x3(elem_normals)), None) => {
                direction_mesh
                    .insert_attribute(Mesh::ATTRIBUTE_NORMAL, elem_normals[normal_range].to_vec());
            }
            _ => unreachable!("Element will always have Float32x3 normals"),
        }
    }

    /// Appends the [`Mesh::ATTRIBUTE_UV_0`] values
    /// from an `element mesh` to a `direction mesh`.
    pub(super) fn append_element_uvs(
        attribute_group: usize,
        direction_mesh: &mut Mesh,
        element_mesh: &Mesh,
    ) {
        let uv_range = (attribute_group * 4)..(attribute_group * 4 + 4);
        match (
            element_mesh.attribute(Mesh::ATTRIBUTE_UV_0),
            direction_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0),
        ) {
            (
                Some(VertexAttributeValues::Float32x2(elem_uvs)),
                Some(VertexAttributeValues::Float32x2(dir_uvs)),
            ) => {
                dir_uvs.extend_from_slice(&elem_uvs[uv_range]);
            }
            (Some(VertexAttributeValues::Float32x2(elem_uvs)), None) => {
                direction_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, elem_uvs[uv_range].to_vec());
            }
            _ => unreachable!("Element will always have Float32x2 uvs"),
        }
    }
}
