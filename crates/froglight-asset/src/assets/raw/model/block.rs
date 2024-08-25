use std::collections::BTreeMap;

use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_utils::HashMap;
use froglight_common::Direction;
use glam::{EulerRot, Quat};
use serde::{Deserialize, Serialize};

use crate::assets::{processed::model::ModelTransformIndex, SerdeJsonLoader};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset::<BlockModelDefinition>();
    app.init_asset_loader::<SerdeJsonLoader<BlockModelDefinition>>();

    app.register_type::<BlockModelDefinition>()
        .register_type::<Handle<BlockModelDefinition>>()
        .register_type_data::<Handle<BlockModelDefinition>, ReflectHandle>();
}

/// A definition for a block model.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Asset, Reflect)]
#[reflect(Default, Asset, Serialize, Deserialize)]
pub struct BlockModelDefinition {
    /// The parent model to inherit from.
    pub parent: Option<String>,

    /// Whether to apply ambient occlusion.
    #[serde(default, rename = "ambientocclusion", skip_serializing_if = "Option::is_none")]
    pub ambient_occlusion: Option<bool>,

    /// The display transforms for the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<BTreeMap<ModelTransformIndex, DefinitionTransform>>,

    /// The textures for the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub textures: Option<HashMap<String, ResourceOrVariable>>,

    /// The elements for the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<DefinitionElement>>,
}

impl BlockModelDefinition {
    /// The default value for the ambient occlusion
    pub const DEFAULT_AMBIENT_OCCLUSION: bool = true;
}

/// A transform for a model.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub struct DefinitionTransform {
    /// The rotation of the transform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<[f32; 3]>,
    /// The translation of the transform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translation: Option<[f32; 3]>,
    /// The scale of the transform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f32; 3]>,
}

impl DefinitionTransform {
    /// The default transform value.
    pub const DEFAULT_TRANSFORM: [f32; 3] = [0.0, 0.0, 0.0];

    /// The transform rotation value.
    #[must_use]
    pub fn rotation(&self) -> [f32; 3] { self.rotation.unwrap_or(Self::DEFAULT_TRANSFORM) }

    /// The transform translation value.
    #[must_use]
    pub fn translation(&self) -> [f32; 3] { self.translation.unwrap_or(Self::DEFAULT_TRANSFORM) }

    /// The transform scale value.
    #[must_use]
    pub fn scale(&self) -> [f32; 3] { self.scale.unwrap_or(Self::DEFAULT_TRANSFORM) }
}

impl From<DefinitionTransform> for bevy_transform::components::Transform {
    fn from(value: DefinitionTransform) -> Self { Self::from(&value) }
}
impl From<&DefinitionTransform> for bevy_transform::components::Transform {
    fn from(value: &DefinitionTransform) -> Self {
        bevy_transform::components::Transform {
            translation: value.translation().into(),
            scale: value.scale().into(),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                value.rotation()[0].to_radians(),
                value.rotation()[1].to_radians(),
                value.rotation()[2].to_radians(),
            ),
        }
    }
}

/// A reference to a resource or another variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Serialize, Deserialize)]
pub enum ResourceOrVariable {
    /// A key to a resource.
    Resource(String),
    /// A reference to another variable.
    Variable(String),
}

/// An element of a model.
#[derive(Debug, Default, Clone, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub struct DefinitionElement {
    /// The starting corner of the element.
    pub from: [f32; 3],
    /// The ending corner of the element.
    pub to: [f32; 3],

    /// The rotation of the element.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<ElementRotation>,

    /// Whether to shade the element.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shade: Option<bool>,

    /// The faces of the element.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub faces: BTreeMap<Direction, ElementFace>,
}

impl DefinitionElement {
    /// The default value for the shade.
    pub const DEFAULT_SHADE: bool = true;

    /// Whether to shade the element.
    #[must_use]
    pub fn shade(&self) -> bool { self.shade.unwrap_or(Self::DEFAULT_SHADE) }
}

/// The rotation of a [`DefinitionElement`].
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ElementRotation {
    /// The origin of rotation.
    pub origin: [f32; 3],

    /// The axis of rotation.
    pub axis: RotationAxis,

    /// The angle of rotation in degrees.
    ///
    /// Between `-45` and `45` in increments of 22.5
    pub angle: f32,

    /// Whether to rescale the model after rotation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rescale: Option<bool>,
}

impl ElementRotation {
    /// The default value for the rescale.
    pub const DEFAULT_RESCALE: bool = false;

    /// Whether to rescale the model after rotation.
    #[must_use]
    pub fn rescale(&self) -> bool { self.rescale.unwrap_or(Self::DEFAULT_RESCALE) }
}

/// The axis of rotation for an [`ElementRotation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Serialize, Deserialize)]
pub enum RotationAxis {
    /// The x-axis.
    X,
    /// The y-axis.
    Y,
    /// The z-axis.
    Z,
}

/// A face of a [`DefinitionElement`].
#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
pub struct ElementFace {
    /// The uv coordinates for the face.
    ///
    /// In the format: `[x1, y1, x2, y2]`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv: Option<[f32; 4]>,

    /// The texture for the face.
    pub texture: ResourceOrVariable,

    /// The culling direction for the face.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cullface: Option<Direction>,

    /// The rotation of the texture.
    pub rotation: Option<u32>,

    /// The tint index for the face.
    #[serde(default, rename = "tintindex", skip_serializing_if = "Option::is_none")]
    pub tint_index: Option<i32>,
}

impl ElementFace {
    /// The default value for the rotation.
    pub const DEFAULT_ROTATION: u32 = 0;
    /// The default value for the tint index.
    pub const DEFAULT_TINT_INDEX: i32 = -1;

    /// The uv coordinates of the face.
    ///
    /// Uses the [`DefinitionElement`]'s `from` and `to`
    /// values if not specified.
    // TODO: Does this care about the direction of the current face?
    #[must_use]
    pub fn uv(&self, element: &DefinitionElement) -> [f32; 4] {
        self.uv.unwrap_or_else(|| [element.from[0], element.from[1], element.to[0], element.to[1]])
    }

    /// The cullface of the current face.
    ///
    /// Uses the opposite of the current face if not specified.
    #[must_use]
    pub fn cullface(&self, current: &Direction) -> Direction {
        self.cullface.unwrap_or(current.opposite())
    }

    /// The rotation of the face.
    #[must_use]
    pub fn rotation(&self) -> u32 { self.rotation.unwrap_or(Self::DEFAULT_ROTATION) }

    /// The tint index of the face.
    #[must_use]
    pub fn tint_index(&self) -> i32 { self.tint_index.unwrap_or(Self::DEFAULT_TINT_INDEX) }
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
            Ok(ResourceOrVariable::Resource(value))
        }
    }
}

impl Serialize for RotationAxis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RotationAxis::X => 'x'.serialize(serializer),
            RotationAxis::Y => 'y'.serialize(serializer),
            RotationAxis::Z => 'z'.serialize(serializer),
        }
    }
}
impl<'de> Deserialize<'de> for RotationAxis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match char::deserialize(deserializer)? {
            'x' | 'X' => Ok(RotationAxis::X),
            'y' | 'Y' => Ok(RotationAxis::Y),
            'z' | 'Z' => Ok(RotationAxis::Z),
            _ => Err(serde::de::Error::custom("invalid rotation axis")),
        }
    }
}
