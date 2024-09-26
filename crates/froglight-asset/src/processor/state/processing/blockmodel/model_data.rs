use bevy_asset::{Assets, UntypedHandle};
use bevy_log::error;
use bevy_math::{prelude::Rectangle, Dir3};
use bevy_render::{
    mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    texture::Image,
};
use bevy_transform::components::Transform;
use bevy_utils::HashMap;
use froglight_common::{Direction, ResourceKey};
use glam::{FloatExt, Vec2, Vec3, Vec3Swizzles};

use super::BlockModelProcessor;
use crate::{
    assets::{
        processed::{model::ModelTransformIndex, BlockAtlas, FallbackTexture},
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

    /// Creates a [`Mesh`] for an [`ElementFace`].
    #[must_use]
    pub(super) fn create_face_mesh(
        _face: &ElementFace,
        element: &DefinitionElement,
        direction: Direction,
    ) -> Mesh {
        Mesh::from(Self::get_rectangle(element, direction))
            .transformed_by(
                Transform::from_translation(
                    // Center the face on the element
                    Vec3::from(element.from).midpoint(Vec3::from(element.to)) - Vec3::splat(8.0),
                )
                // TODO: Rotate the face based on the direction
                .looking_to(
                    match direction {
                        Direction::North | Direction::South => direction.to_axis().zxy().as_vec3(),
                        Direction::East | Direction::West => direction.to_axis().yxz().as_vec3(),
                        Direction::Up | Direction::Down => direction.opposite().to_axis().as_vec3(),
                    },
                    match direction {
                        Direction::North | Direction::South => Dir3::X,
                        Direction::East | Direction::West => Dir3::Y,
                        Direction::Up | Direction::Down => Dir3::Z,
                    },
                ),
            )
            // Scale the face to the correct size
            .scaled_by(Vec3::splat(1.0 / 16.0))
    }

    /// Creates a [`Rectangle`] for an [`ElementFace`] based on the direction.
    #[must_use]
    fn get_rectangle(element: &DefinitionElement, direction: Direction) -> Rectangle {
        let (from, to) = (Vec3::from(element.from), Vec3::from(element.to));
        let (from, to): (Vec2, Vec2) = match direction {
            Direction::Up => (from.xz(), to.xz()),
            Direction::Down => (to.xz(), from.xz()),
            Direction::North => (from.yz(), to.yz()),
            Direction::South => (to.yz(), from.yz()),
            Direction::East => (to.xy(), from.xy()),
            Direction::West => (from.xy(), to.xy()),
        };
        Rectangle::from_corners(from, to)
    }

    /// Appends the positions of an [`ElementFace`] to a [`Mesh`].
    pub(super) fn append_element_positions(
        _face: &ElementFace,
        _element: &DefinitionElement,
        _face_mesh: &mut Mesh,
    ) {
    }

    /// Appends the normals of an [`ElementFace`] to a [`Mesh`].
    pub(super) fn append_element_normals(
        _face: &ElementFace,
        _element: &DefinitionElement,
        _texture: Option<&UntypedHandle>,
        _catalog: &AssetCatalog,
        _face_mesh: &mut Mesh,
    ) {
    }

    /// Appends the UVs of an [`ElementFace`] to a [`Mesh`].
    pub(super) fn append_element_uvs(
        face: &ElementFace,
        element: &DefinitionElement,
        texture: Option<&UntypedHandle>,
        atlas: &BlockAtlas,
        catalog: &AssetCatalog,
        direction: Direction,
        face_mesh: &mut Mesh,
    ) {
        // Get the texture index in the atlas, or the fallback texture if missing
        let atlas_index: usize = texture
            .and_then(|h| atlas.layout().get_texture_index(h.id().typed_debug_checked()))
            .unwrap_or_else(|| {
                // Log an error if the texture is missing
                if let Some(texture) = texture {
                    if let Some(path) = texture.path() {
                        error!("BlockModelProcessor: BlockAtlas missing texture, \"{path}\"");
                    } else {
                        error!(
                            "BlockModelProcessor: BlockAtlas missing texture, {:?}",
                            texture.id()
                        );
                    }
                }

                // Get the index of the fallback texture
                catalog
                    .get_untyped::<Image>(&FallbackTexture::ASSET_KEY)
                    .and_then(|h| atlas.layout().get_texture_index(h.id().typed_debug_checked()))
                    .expect("BlockAtlas missing FallbackTexture")
            });

        let atlas_rect = atlas.layout().textures[atlas_index].as_rect();
        let atlas_size = atlas.layout().size.as_vec2();

        // Order: x1, x2, y1, y2
        let mut face_uvs = face.uv(element, direction);
        // Apply the face's rotation to the UVs
        face_uvs.rotate_right(face.rotation() as usize / 90 % 4);

        // Remap the UVs to the texture atlas
        let mut uvs = Vec::with_capacity(4);
        for index in 0..4 {
            // Push UVs in this order,
            let (u, v) = match index {
                // X1, Y1
                0 => (face_uvs[0], face_uvs[2]),
                // X1, Y2
                1 => (face_uvs[0], face_uvs[3]),
                // X2, Y2
                2 => (face_uvs[1], face_uvs[3]),
                // X2, Y1
                3 => (face_uvs[1], face_uvs[2]),
                _ => unreachable!(),
            };
            uvs.push([
                u.remap(
                    0.0,
                    16.0,
                    atlas_rect.min.x / atlas_size.x,
                    atlas_rect.max.x / atlas_size.x,
                ),
                v.remap(
                    0.0,
                    16.0,
                    atlas_rect.min.y / atlas_size.y,
                    atlas_rect.max.y / atlas_size.y,
                ),
            ]);
        }
        face_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    }
}
