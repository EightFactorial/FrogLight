use std::collections::BTreeMap;

use bevy_asset::{Asset, ReflectAsset};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_transform::components::Transform;
use bevy_utils::HashMap;
use froglight_common::{Direction, ResourceKey};
use glam::{EulerRot, Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::assets::processed::ModelTransformIndex;

/// A block model definition.
#[derive(Debug, Default, Clone, PartialEq, Reflect, Asset, Serialize, Deserialize)]
#[reflect(Default, Asset, Serialize, Deserialize)]
pub struct BlockModelDefinition {
    /// The parent of this block model.
    pub parent: Option<ResourceKey>,

    /// The ambient occlusion of the block model.
    #[serde(default, rename = "ambientocclusion")]
    pub ambient_occlusion: Option<bool>,

    /// The display transforms of the block model.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub display: HashMap<ModelTransformIndex, DefinitionTransform>,

    /// The textures of the block model.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub textures: HashMap<String, ResourceOrVariable>,

    /// The elements of the block model.
    ///
    /// If `Some`, do not include elements from the parent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<DefinitionElement>>,
}

impl BlockModelDefinition {
    /// The default value for ambient occlusion.
    pub const DEFAULT_AMBIENT_OCCLUSION: bool = true;
}

// --- DefinitionTransform ---

#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub struct DefinitionTransform {
    #[serde(default, skip_serializing_if = "DefinitionTransform::default_transform")]
    pub rotation: [f32; 3],
    #[serde(default, skip_serializing_if = "DefinitionTransform::default_transform")]
    pub translation: [f32; 3],
    #[serde(default, skip_serializing_if = "DefinitionTransform::default_transform")]
    pub scale: [f32; 3],
}

impl DefinitionTransform {
    fn default_transform(transform: &[f32; 3]) -> bool {
        transform.iter().all(|&f| f.abs() < f32::EPSILON)
    }
}

impl From<DefinitionTransform> for Transform {
    fn from(value: DefinitionTransform) -> Self {
        Transform {
            translation: Vec3::from(value.translation),
            scale: Vec3::from(value.scale),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                value.rotation[0].to_radians(),
                value.rotation[1].to_radians(),
                value.rotation[2].to_radians(),
            ),
        }
    }
}

// --- DefinitionElement ---

#[derive(Debug, Default, Clone, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub struct DefinitionElement {
    pub from: [f32; 3],
    pub to: [f32; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<ElementRotation>,
    #[serde(default, skip_serializing_if = "DefinitionElement::is_shaded")]
    pub shade: bool,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub faces: BTreeMap<Direction, ElementFace>,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
impl DefinitionElement {
    pub const DEFAULT_SHADE: bool = true;
    fn is_shaded(shade: &bool) -> bool { !shade }
}

// --- ElementRotation ---

#[derive(Debug, Clone, Copy, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ElementRotation {
    /// The origin of rotation.
    pub origin: [f32; 3],

    /// The axis of rotation.
    ///
    /// Can be `x`, `y`, or `z`
    pub axis: char,

    /// The angle of rotation.
    ///
    /// Between `-45` and `45` in increments of 22.5
    pub angle: f32,

    #[serde(default, skip_serializing_if = "ElementRotation::not_rescaled")]
    pub rescale: bool,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
impl ElementRotation {
    pub const DEFAULT_RESCALE: bool = false;
    fn not_rescaled(rescale: &bool) -> bool { !rescale }
}

// --- ElementFace ---

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ElementFace {
    /// If `None`, use the position of the face in the [`DefinitionElement`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv: Option<[f32; 4]>,

    pub texture: ResourceOrVariable,

    /// The cullface of the element face.
    ///
    /// If `None`, use the position of the face in the [`DefinitionElement`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cullface: Option<Direction>,

    #[serde(default, skip_serializing_if = "ElementFace::default_rotation")]
    pub rotation: u32,
    #[serde(default, skip_serializing_if = "ElementFace::default_tintindex")]
    pub tintindex: i32,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
impl ElementFace {
    fn default_rotation(rotation: &u32) -> bool { *rotation == 0 }
    fn default_tintindex(tintindex: &i32) -> bool { *tintindex == -1 }

    // TODO: Check if this is correct
    pub fn uvs_from(&self) -> Option<[[f32; 2]; 4]> {
        self.uv.map(|mut uv| {
            uv.rotate_right(((self.rotation / 90) as usize) % 4);
            [
                [uv[0] / 16f32, uv[1] / 16f32],
                [uv[2] / 16f32, uv[1] / 16f32],
                [uv[2] / 16f32, uv[3] / 16f32],
                [uv[0] / 16f32, uv[3] / 16f32],
            ]
        })
    }
}

// --- ResourceOrVariable ---

/// A resource or variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum ResourceOrVariable {
    /// A [`ResourceKey`] that refers to a resource.
    Resource(ResourceKey),
    /// A reference to a variable.
    ///
    /// Variables are prefixed with a `#`.
    Variable(String),
}

impl Serialize for ResourceOrVariable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ResourceOrVariable::Resource(resource) => resource.serialize(serializer),
            ResourceOrVariable::Variable(variable) => format!("#{variable}").serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ResourceOrVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if let Some(variable) = value.strip_prefix('#') {
            Ok(ResourceOrVariable::Variable(variable.to_string()))
        } else {
            Ok(ResourceOrVariable::Resource(ResourceKey::new(value)))
        }
    }
}
