//! [`BlockStateDefinition`] and related types.
#![allow(clippy::used_underscore_binding)]

use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_derive::{Deref, DerefMut};
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_utils::HashMap;
use serde::{ser::SerializeMap, Deserialize, Serialize};

use crate::assets::SerdeJsonLoader;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset::<BlockStateDefinition>();
    app.init_asset_loader::<SerdeJsonLoader<BlockStateDefinition>>();

    app.register_type::<BlockStateDefinition>()
        .register_type::<Handle<BlockStateDefinition>>()
        .register_type_data::<Handle<BlockStateDefinition>, ReflectHandle>();
}

/// A block state definition.
///
/// Defines what models to use for a block in different states.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Asset, Reflect)]
#[reflect(Serialize, Deserialize, Asset)]
#[serde(untagged)]
pub enum BlockStateDefinition {
    /// A list of states and models for each state.
    Variants {
        /// A list of variants and their models.
        variants: HashMap<VariantKey, StateModelDefinitions>,
    },
    /// A list of parts and conditions for each part.
    MultiPart {
        /// A list of parts and their conditions.
        multipart: Vec<BlockStateMultiPart>,
    },
}

/// A key for a [`BlockStateVariant`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct VariantKey(Vec<(String, Vec<String>)>);

/// One or more [`StateModelDefinition`]s.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[serde(untagged)]
pub enum StateModelDefinitions {
    /// A single model.
    Single(StateModelDefinition),
    /// Multiple models.
    Multiple(Vec<StateModelDefinition>),
}

impl StateModelDefinitions {
    /// Returns all models as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[StateModelDefinition] {
        match self {
            StateModelDefinitions::Single(model) => std::slice::from_ref(model),
            StateModelDefinitions::Multiple(models) => models,
        }
    }
}

/// A block state variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct StateModelDefinition {
    /// The model for the variant.
    pub model: String,

    /// The rotation of the model in the x-axis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<u32>,

    /// The rotation of the model in the y-axis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<u32>,

    /// Whether the UVs should be locked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uvlock: Option<bool>,

    /// The weight of the variant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
}

impl StateModelDefinition {
    /// The default value for the `x` field.
    pub const DEFAULT_X: u32 = 0;

    /// The rotation of the model in the x-axis.
    #[must_use]
    pub fn x(&self) -> u32 { self.x.unwrap_or(Self::DEFAULT_X) }

    /// The default value for the `y` field.
    pub const DEFAULT_Y: u32 = 0;

    /// The rotation of the model in the y-axis.
    #[must_use]
    pub fn y(&self) -> u32 { self.y.unwrap_or(Self::DEFAULT_Y) }

    /// The default value for the `uvlock` field.
    pub const DEFAULT_UVLOCK: bool = false;

    /// Whether the UVs should be locked.
    #[must_use]
    pub fn uvlock(&self) -> bool { self.uvlock.unwrap_or(Self::DEFAULT_UVLOCK) }

    /// The default value for the `weight` field.
    pub const DEFAULT_WEIGHT: u32 = 1;

    /// The weight of the variant.
    #[must_use]
    pub fn weight(&self) -> u32 { self.weight.unwrap_or(Self::DEFAULT_WEIGHT) }
}

/// A part of a block state multipart.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct BlockStateMultiPart {
    /// The condition for the part.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<MultiPartWhen>,
    /// The model for the part.
    pub apply: StateModelDefinitions,
}

/// Conditions for a block state multipart.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub enum MultiPartWhen {
    /// If any of the conditions are met.
    #[serde(rename = "OR")]
    Or(Vec<MultiPartCondition>),
    /// If all of the conditions are met.
    #[serde(rename = "AND")]
    And(Vec<MultiPartCondition>),
    /// If the condition is met.
    #[serde(untagged)]
    Single(MultiPartCondition),
}

/// A condition for a block state multipart.
#[derive(Debug, Clone, PartialEq, Deref, DerefMut, Reflect)]
pub struct MultiPartCondition(VariantKey);

impl Serialize for VariantKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut string = String::new();
        for (index, (key, values)) in self.iter().enumerate() {
            string.push_str(key);
            string.push('=');

            for (index, value) in values.iter().enumerate() {
                string.push_str(value);

                if index != values.len() - 1 {
                    string.push('|');
                }
            }

            if index != self.0.len() - 1 {
                string.push(',');
            }
        }
        string.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for VariantKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut vec = Vec::new();
        let string = String::deserialize(deserializer)?;

        if !string.is_empty() {
            for key_values in string.split(',') {
                let (key, values) = key_values.split_once('=').ok_or_else(|| {
                    serde::de::Error::custom(
                        "expected comma separated key=value pairs for VariantKey",
                    )
                })?;
                vec.push((key.to_string(), values.split('|').map(String::from).collect()));
            }
        }

        Ok(VariantKey(vec))
    }
}

impl Serialize for MultiPartCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ser = serializer.serialize_map(Some(self.len()))?;
        for (key, values) in self.iter() {
            ser.serialize_entry(key, &values.join("|"))?;
        }
        ser.end()
    }
}
impl<'de> Deserialize<'de> for MultiPartCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MultiPartCondition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map of key-value pairs")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut vec = Vec::with_capacity(map.size_hint().unwrap_or(0));
                while let Some((key, value)) = map.next_entry::<String, String>()? {
                    vec.push((key, value.split('|').map(String::from).collect()));
                }
                Ok(MultiPartCondition(VariantKey(vec)))
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}
