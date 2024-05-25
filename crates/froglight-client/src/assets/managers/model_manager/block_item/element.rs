use bevy::reflect::Reflect;
use froglight_assets::assets::model::{
    ElementFace as DefinitionElementFace, ElementRotation, ModelElement as DefinitionModelElement,
    ModelFace,
};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

use crate::assets::AssetManager;

/// A model element
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ResolvedModelElement {
    /// The starting point of the cube
    ///
    /// Must be between `-16` and `32`
    pub from: [f32; 3],

    /// The ending point of the cube
    ///
    /// Must be between `-16` and `32`
    pub to: [f32; 3],

    /// The rotation of the cube
    pub rotation: Option<ElementRotation>,

    /// Whether to shade the cube
    pub shade: bool,

    /// The faces of the cube
    ///
    /// Indexed via [`ModelFace`].
    pub faces: [ResolvedElementFace; 6],
}

impl ResolvedModelElement {
    /// Resolves a [`DefinitionModelElement`] into a [`ResolvedModelElement`].
    #[must_use]
    pub fn resolve_from(
        key: &ResourceKey,
        element: &DefinitionModelElement,
        textures: &HashMap<&String, &String>,
    ) -> Option<Self> {
        let faces = std::array::try_from_fn(|index| {
            // Get the face definition, return `None` if it does not exist
            let face = ModelFace::from_index(index);
            let face_def = element.faces.get(&face)?;

            Some(ResolvedElementFace {
                // Use the UVs if they are defined, otherwise use the default UVs
                uv: face_def
                    .uv
                    .unwrap_or_else(|| Self::default_uvs(face, &element.from, &element.to)),
                // Use the texture key to get the texture, or return the fallback texture
                texture: Self::resolve_texture(key, face_def, textures)
                    .unwrap_or(AssetManager::FALLBACK_TEXTURE),
                cullface: face_def.cullface.unwrap_or(face),
                rotation: face_def.rotation,
                tint_index: face_def.tint_index,
            })
        })?;

        Some(Self {
            from: element.from,
            to: element.to,
            rotation: element.rotation,
            shade: element.shade,
            faces,
        })
    }

    /// Returns the default UVs for a face, given the `from` and `to`
    /// coordinates
    ///
    /// The UVs are returned in the order `[x1, y1, x2, y2]`
    // TODO: Check if the UVs are correct
    #[must_use]
    fn default_uvs(face: ModelFace, from: &[f32; 3], to: &[f32; 3]) -> [f32; 4] {
        match face {
            ModelFace::Down | ModelFace::Up => [from[0], from[2], to[0], to[2]],
            ModelFace::North | ModelFace::South => [from[0], from[1], to[0], to[1]],
            ModelFace::West | ModelFace::East => [from[2], from[1], to[2], to[1]],
        }
    }

    /// Attempt to resolve a texture key into a [`ResourceKey`].
    ///
    /// Fails if the texture key is not found in the `textures` map.
    #[must_use]
    fn resolve_texture(
        key: &ResourceKey,
        face_def: &DefinitionElementFace,
        textures: &HashMap<&String, &String>,
    ) -> Option<ResourceKey> {
        let mut texture = face_def.texture.as_str();

        // Resolve the texture key until an actual texture is found
        while texture.starts_with('#') {
            if let Some(&new_texture) = textures.get(&texture[1..].to_string()) {
                texture = new_texture;
            } else {
                #[cfg(debug_assertions)]
                {
                    bevy::log::error!(
                        "Failed to resolve texture \"{}\" for \"{key}\"",
                        face_def.texture.as_str()
                    );
                    bevy::log::debug!("Available textures for \"{key}\": {textures:?}");
                }
                #[cfg(not(debug_assertions))]
                bevy::log::error!("Failed to find texture for \"{key}\"");
                return None;
            }
        }

        // Try to create a `ResourceKey` from the texture
        ResourceKey::try_new(texture).ok()
    }
}

/// A block model element face
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ResolvedElementFace {
    /// The area of the texture to use
    ///
    /// The UVs are in the order `[x1, y1, x2, y2]`
    pub uv: [f32; 4],

    /// The texture to use
    pub texture: ResourceKey,

    /// Cull this face when a block is placed against it
    ///
    /// Defaults to the side defined by the face
    pub cullface: ModelFace,

    /// Rotate the texture by this many degrees
    ///
    /// Must be a multiple of 90
    pub rotation: i32,

    /// If the face should be tinted using a color index
    ///
    /// These are hardcoded only for certain blocks
    pub tint_index: i32,
}
