use froglight_assets::assets::{
    model::{DisplayPosition, ModelDisplayTransform},
    BlockModelDefinition, ModelDefinition,
};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

use crate::assets::model_manager::ResolvedModelElement;

/// Recursively get the ambient occlusion for the model.
///
/// Checks the current model definition, then the parent model definition.
#[must_use]
#[allow(unused_variables)]
pub(super) fn recursive_occlusion(
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
                return recursive_occlusion(parent, parent_def, definitions);
            }

            #[cfg(debug_assertions)]
            bevy::log::error!("Parent is not a block model: \"{key}\" -> \"{parent}\"");
        } else {
            #[cfg(debug_assertions)]
            bevy::log::warn!("No parent for block model: \"{key}\" -> \"{parent}\"");
        }
    }

    None
}

/// Get all of the model transforms for the model.
#[must_use]
pub(super) fn recursive_transforms(
    key: &ResourceKey,
    def: &BlockModelDefinition,
    definitions: &HashMap<ResourceKey, ModelDefinition>,
) -> [ModelDisplayTransform; 7] {
    std::array::from_fn(|position_index| {
        recurse_transform(DisplayPosition::from_index(position_index), key, def, definitions)
            .unwrap_or_default()
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
                    return recurse_transform(display, parent, parent_def, definitions);
                }

                #[cfg(debug_assertions)]
                bevy::log::error!("Parent is not a block model: \"{key}\" -> \"{parent}\"");
            } else {
                #[cfg(debug_assertions)]
                bevy::log::warn!("No parent for block model: \"{key}\" -> \"{parent}\"");
            }
        }

        None
    })
}

/// Get the elements for the model.
pub(super) fn recursive_elements(
    key: &ResourceKey,
    def: &BlockModelDefinition,
    definitions: &HashMap<ResourceKey, ModelDefinition>,
) -> Vec<ResolvedModelElement> {
    let mut textures = HashMap::new();
    if let Some(def_textures) = &def.textures {
        textures.extend(&def_textures.0);
    }
    recurse_elements(key, def, &mut textures, definitions)
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
            .map(|element| ResolvedModelElement::resolve_from(key, element, textures))
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
                    return recurse_elements(key, parent_def, textures, definitions);
                }

                #[cfg(debug_assertions)]
                bevy::log::error!("Parent is not a block model: \"{key}\" -> \"{parent}\"");
            } else {
                #[cfg(debug_assertions)]
                bevy::log::warn!("No parent for block model: \"{key}\" -> \"{parent}\"");
            }
        }

        Vec::new()
    }
}
