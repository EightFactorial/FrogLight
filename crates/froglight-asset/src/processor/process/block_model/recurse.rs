use bevy_asset::{AssetId, Assets};
use bevy_render::texture::Image;
use bevy_transform::components::Transform;
use bevy_utils::HashMap;

use super::BlockModelState;
use crate::{
    assets::{
        processed::ModelTransformIndex,
        unprocessed::{
            block_definition::{DefinitionElement, ElementFace, ResourceOrVariable},
            BlockModelDefinition,
        },
    },
    AssetCatalog,
};

impl BlockModelState {
    /// Gets the [`Transform`] for a [`BlockModelDefinition`]'s
    /// [`ModelTransformIndex`].
    pub(super) fn recurse_for_transform(
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

    /// Gets the [`DefinitionElement`]s for a [`BlockModelDefinition`].
    pub(super) fn recurse_for_elements<'a>(
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

    /// Get's the texture for an [`ElementFace`].
    pub(super) fn get_texture(
        catalog: &AssetCatalog,
        element_face: &ElementFace,
        definition: &BlockModelDefinition,
        definitions: &Assets<BlockModelDefinition>,
    ) -> Option<AssetId<Image>> {
        match &element_face.texture {
            ResourceOrVariable::Resource(key) => catalog.get::<Image>(key),
            ResourceOrVariable::Variable(var) => Self::recurse_for_resource(
                catalog,
                var.to_owned(),
                definition,
                definitions,
                &mut HashMap::default(),
            ),
        }
    }

    /// Get's the [`AssetId`] for a resource.
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
}
