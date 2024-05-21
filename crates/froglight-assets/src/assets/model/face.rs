use bevy_app::App;
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ElementFace>().register_type::<ModelFace>();
}

/// A block model element face
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct ElementFace {
    /// The area of the texture to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv: Option<[f32; 4]>,

    /// The texture to use
    ///
    /// Prefix with `#` to reference a texture from the `textures` field
    pub texture: String,

    /// Cull this face when a block is placed against it
    ///
    /// Defaults to the side defined by the face
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cullface: Option<ModelFace>,

    /// Rotate the texture by this many degrees
    ///
    /// Must be a multiple of 90
    #[serde(default, skip_serializing_if = "ElementFace::is_rotation_default")]
    pub rotation: i32,

    /// If the face should be tinted using a color index
    ///
    /// These are hardcoded only for certain blocks
    #[serde(
        default = "ElementFace::tint_index_default",
        skip_serializing_if = "ElementFace::tint_index_is_default"
    )]
    pub tint_index: i32,
}

impl ElementFace {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    const fn is_rotation_default(rotation: &i32) -> bool { *rotation == 0 }

    const fn tint_index_default() -> i32 { -1 }
    #[allow(clippy::trivially_copy_pass_by_ref)]
    const fn tint_index_is_default(tint_index: &i32) -> bool {
        *tint_index == Self::tint_index_default()
    }
}

/// A face in a model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelFace {
    /// The bottom face
    #[serde(alias = "bottom")]
    Down,
    /// The top face
    Up,
    /// The north face
    North,
    /// The south face
    South,
    /// The west face
    West,
    /// The east face
    East,
}
