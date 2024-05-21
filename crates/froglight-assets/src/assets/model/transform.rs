use bevy_app::App;
use bevy_derive::{Deref, DerefMut};
use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ModelDisplayTransforms>()
        .register_type::<DisplayPosition>()
        .register_type::<ModelDisplayTransform>();
}

/// The display settings for the model in various contexts
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Deref, DerefMut, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub struct ModelDisplayTransforms(pub HashMap<DisplayPosition, ModelDisplayTransform>);

/// A model display position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisplayPosition {
    /// Third person right hand
    #[serde(rename = "thirdperson_righthand")]
    ThirdPersonRightHand,
    /// Third person left hand
    #[serde(rename = "thirdperson_lefthand")]
    ThirdPersonLeftHand,
    /// First person right hand
    #[serde(rename = "firstperson_righthand")]
    FirstPersonRightHand,
    /// First person left hand
    #[serde(rename = "firstperson_lefthand")]
    FirstPersonLeftHand,
    /// GUI
    Gui,
    /// Head
    Head,
    /// Ground
    Ground,
    /// Fixed
    Fixed,
}

/// A model display transform
///
/// Translations are applied before rotations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub struct ModelDisplayTransform {
    /// Rotation
    #[serde(
        default = "ModelDisplayTransform::default_rotation",
        skip_serializing_if = "ModelDisplayTransform::is_default_rotation"
    )]
    pub rotation: [f32; 3],
    /// Translation
    ///
    /// Clamped between `-80` and `80`
    #[serde(
        default = "ModelDisplayTransform::default_translation",
        skip_serializing_if = "ModelDisplayTransform::is_default_translation"
    )]
    pub translation: [f32; 3],
    /// Scale
    ///
    /// Maximum value of `4`
    #[serde(
        default = "ModelDisplayTransform::default_scale",
        skip_serializing_if = "ModelDisplayTransform::is_default_scale"
    )]
    pub scale: [f32; 3],
}

impl Default for ModelDisplayTransform {
    fn default() -> Self {
        Self { rotation: [0.0, 0.0, 0.0], translation: [0.0, 0.0, 0.0], scale: [1.0, 1.0, 1.0] }
    }
}

impl ModelDisplayTransform {
    const fn default_rotation() -> [f32; 3] { [0.0, 0.0, 0.0] }
    fn is_default_rotation([x, y, z]: &[f32; 3]) -> bool {
        (x - 0.0 < f32::EPSILON) && (y - 0.0 < f32::EPSILON) && (z - 0.0 < f32::EPSILON)
    }

    const fn default_translation() -> [f32; 3] { [0.0, 0.0, 0.0] }
    fn is_default_translation([x, y, z]: &[f32; 3]) -> bool {
        (x - 0.0 < f32::EPSILON) && (y - 0.0 < f32::EPSILON) && (z - 0.0 < f32::EPSILON)
    }

    const fn default_scale() -> [f32; 3] { [1.0, 1.0, 1.0] }
    fn is_default_scale([x, y, z]: &[f32; 3]) -> bool {
        (x - 1.0 < f32::EPSILON) && (y - 1.0 < f32::EPSILON) && (z - 1.0 < f32::EPSILON)
    }
}
