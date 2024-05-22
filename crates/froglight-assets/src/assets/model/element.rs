use bevy_app::App;
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use super::{ElementFace, ModelFace};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ModelElement>()
        .register_type::<ElementRotation>()
        .register_type::<ModelAxis>();
}

/// A model element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<ElementRotation>,

    /// Whether to shade the cube
    #[serde(
        default = "ModelElement::shade_default",
        skip_serializing_if = "ModelElement::is_default_shade"
    )]
    pub shade: bool,

    /// The faces of the cube
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub faces: HashMap<ModelFace, ElementFace>,
}

impl ModelElement {
    /// The default shade value
    #[must_use]
    pub const fn shade_default() -> bool { true }
    #[allow(clippy::trivially_copy_pass_by_ref)]
    const fn is_default_shade(b: &bool) -> bool { *b == Self::shade_default() }
}

/// A model element rotation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct ElementRotation {
    /// The origin of rotation
    pub origin: [f32; 3],

    /// The axis of rotation
    pub axis: ModelAxis,

    /// The angle of rotation
    ///
    /// Must be between `-45` and `45`,
    /// in increments of `22.5`
    pub angle: f32,

    /// Whether to scale the faces across the whole block
    #[serde(default, skip_serializing_if = "ElementRotation::is_default_rescale")]
    pub rescale: bool,
}

impl ElementRotation {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    const fn is_default_rescale(rescale: &bool) -> bool { !*rescale }
}

/// An axis in a model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelAxis {
    /// The X axis
    X,
    /// The Y axis
    Y,
    /// The Z axis
    Z,
}
