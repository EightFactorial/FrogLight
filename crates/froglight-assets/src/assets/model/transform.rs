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

impl DisplayPosition {
    /// Returns all display positions as a slice
    #[must_use]
    pub const fn as_slice() -> &'static [Self] {
        &[
            Self::ThirdPersonRightHand,
            Self::ThirdPersonLeftHand,
            Self::FirstPersonRightHand,
            Self::FirstPersonLeftHand,
            Self::Gui,
            Self::Head,
            Self::Ground,
            Self::Fixed,
        ]
    }
}

/// A model display transform
///
/// Translations are applied before rotations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub struct ModelDisplayTransform {
    /// Rotation
    #[serde(
        default = "ModelDisplayTransform::rotation_default",
        skip_serializing_if = "ModelDisplayTransform::is_default_rotation"
    )]
    pub rotation: [f32; 3],
    /// Translation
    ///
    /// Clamped between `-80` and `80`
    #[serde(
        default = "ModelDisplayTransform::translation_default",
        skip_serializing_if = "ModelDisplayTransform::is_default_translation"
    )]
    pub translation: [f32; 3],
    /// Scale
    ///
    /// Maximum value of `4`
    #[serde(
        default = "ModelDisplayTransform::scale_default",
        skip_serializing_if = "ModelDisplayTransform::is_default_scale"
    )]
    pub scale: [f32; 3],
}

impl Default for ModelDisplayTransform {
    fn default() -> Self {
        Self {
            rotation: Self::rotation_default(),
            translation: Self::translation_default(),
            scale: Self::scale_default(),
        }
    }
}

impl ModelDisplayTransform {
    /// The default rotation
    #[must_use]
    pub const fn rotation_default() -> [f32; 3] { [0.0, 0.0, 0.0] }
    fn is_default_rotation([x, y, z]: &[f32; 3]) -> bool {
        (x - 0.0 < f32::EPSILON) && (y - 0.0 < f32::EPSILON) && (z - 0.0 < f32::EPSILON)
    }

    /// The default translation
    #[must_use]
    pub const fn translation_default() -> [f32; 3] { [0.0, 0.0, 0.0] }
    fn is_default_translation([x, y, z]: &[f32; 3]) -> bool {
        (x - 0.0 < f32::EPSILON) && (y - 0.0 < f32::EPSILON) && (z - 0.0 < f32::EPSILON)
    }

    /// The default scale
    #[must_use]
    pub const fn scale_default() -> [f32; 3] { [1.0, 1.0, 1.0] }
    fn is_default_scale([x, y, z]: &[f32; 3]) -> bool {
        (x - 1.0 < f32::EPSILON) && (y - 1.0 < f32::EPSILON) && (z - 1.0 < f32::EPSILON)
    }
}
