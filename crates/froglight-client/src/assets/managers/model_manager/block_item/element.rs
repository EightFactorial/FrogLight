use bevy::reflect::Reflect;
use froglight_assets::assets::model::{
    ElementRotation, ModelElement as ModelDefinitionElement, ModelFace,
};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

/// A model element
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ModelElement {
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
    pub faces: HashMap<ModelFace, ElementFace>,
}

impl ModelElement {
    /// Resolves a [`ModelDefinitionElement`] into a [`ModelElement`].
    #[allow(unused_variables, unreachable_code, clippy::diverging_sub_expression)]
    #[must_use]
    pub fn resolve_from(
        key: &ResourceKey,
        def: &ModelDefinitionElement,
        textures: &HashMap<String, String>,
    ) -> Option<Self> {
        let mut faces = HashMap::new();

        for (face, face_def) in &def.faces {
            faces.insert(
                *face,
                ElementFace {
                    uv: face_def.uv.unwrap_or_else(|| {
                        // Create default UVs, based on the from, to, and face
                        //
                        // TODO: These are probably wrong, fix later
                        match face {
                            ModelFace::Down | ModelFace::Up => {
                                [def.from[0], def.from[2], def.to[0], def.to[2]]
                            }
                            ModelFace::North | ModelFace::South => {
                                [def.from[0], def.from[1], def.to[0], def.to[1]]
                            }
                            ModelFace::West | ModelFace::East => {
                                [def.from[2], def.from[1], def.to[2], def.to[1]]
                            }
                        }
                    }),
                    texture: todo!("Resolve texture from textures map"),
                    cullface: face_def.cullface.unwrap_or(*face),
                    rotation: face_def.rotation,
                    tint_index: face_def.tint_index,
                },
            );
        }

        Some(Self { from: def.from, to: def.to, rotation: def.rotation, shade: def.shade, faces })
    }
}

/// A block model element face
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ElementFace {
    /// The area of the texture to use
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
