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

    // Indices::U32(vec![
    //     0, 1, 2, 2, 3, 0, // front
    //     4, 5, 6, 6, 7, 4, // back
    //     8, 9, 10, 10, 11, 8, // right
    //     12, 13, 14, 14, 15, 12, // left
    //     16, 17, 18, 18, 19, 16, // top
    //     20, 21, 22, 22, 23, 20, // bottom
    // ]);

    /// Returns the positions of the element face.
    ///
    /// Each array is an [x, y, z] coordinate in local space.
    /// The camera coordinate space is right-handed x-right, y-up, z-back.
    /// This means "forward" is -Z.
    #[must_use]
    pub fn positions_from(&self, direction: Direction) -> [[f32; 3]; 4] {
        let [min_x, min_y, min_z] = self.from;
        let [max_x, max_y, max_z] = self.to;

        let mut positions = match direction {
            Direction::North => [
                [min_x, min_y, max_z],
                [max_x, min_y, max_z],
                [max_x, max_y, max_z],
                [min_x, max_y, max_z],
            ],
            Direction::South => [
                [max_x, min_y, min_z],
                [min_x, min_y, min_z],
                [min_x, max_y, min_z],
                [max_x, max_y, min_z],
            ],
            Direction::West => [
                [max_x, min_y, max_z],
                [max_x, min_y, min_z],
                [max_x, max_y, min_z],
                [max_x, max_y, max_z],
            ],
            Direction::East => [
                [min_x, min_y, min_z],
                [min_x, min_y, max_z],
                [min_x, max_y, max_z],
                [min_x, max_y, min_z],
            ],
            Direction::Up => [
                [max_x, max_y, min_z],
                [min_x, max_y, min_z],
                [min_x, max_y, max_z],
                [max_x, max_y, max_z],
            ],
            Direction::Down => [
                [max_x, min_y, max_z],
                [min_x, min_y, max_z],
                [min_x, min_y, min_z],
                [max_x, min_y, min_z],
            ],
        }
        .map(|[x, y, z]| [x / 16f32, y / 16f32, z / 16f32]);

        if let Some(rotation) = self.rotation.as_ref() {
            let axis = match rotation.axis {
                'x' => Vec3::X,
                'y' => Vec3::Y,
                'z' => Vec3::Z,
                _ => unreachable!(),
            };
            let angle = rotation.angle.to_radians();
            let quat = Quat::from_axis_angle(axis, angle);
            let origin = Vec3::from(rotation.origin).normalize();

            for position in &mut positions {
                let pos = Vec3::from(*position);
                let pos = pos - origin;
                let pos = quat.mul_vec3(pos);
                let pos = pos + origin;
                *position = pos.into();
            }
        }

        positions
    }

    /// Returns the UVs of the element face.
    #[must_use]
    pub fn uvs_from(&self, direction: Direction, rotation: u32) -> [[f32; 2]; 4] {
        let [min_x, min_y, min_z] = self.from;
        let [max_x, max_y, max_z] = self.to;

        let mut uvs = match direction {
            Direction::North => [[min_x, min_y], [max_x, min_y], [max_x, max_y], [min_x, max_y]],
            Direction::South => [[min_x, max_y], [max_x, max_y], [max_x, min_y], [min_x, min_y]],
            Direction::West => [[max_z, min_y], [max_z, max_y], [min_z, max_y], [min_z, min_y]],
            Direction::East => [[min_z, min_y], [min_z, max_y], [max_z, max_y], [max_z, min_y]],
            Direction::Up => [[max_x, max_z], [min_x, max_z], [min_x, min_z], [max_x, min_z]],
            Direction::Down => [[max_x, min_z], [min_x, min_z], [min_x, max_z], [max_x, max_z]],
        }
        .map(|[u, v]| [u / 16f32, v / 16f32]);
        uvs.rotate_right((rotation / 90) as usize % 4);

        uvs
    }
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
    /// Between `45 .= -45` in increments of 22.5
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

    pub fn uvs_from(&self) -> Option<[[f32; 2]; 4]> {
        self.uv.map(|mut uv| {
            uv.rotate_right((self.rotation / 90) as usize % 4);
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
