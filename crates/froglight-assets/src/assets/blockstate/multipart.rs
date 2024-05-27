use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

use super::SingleOrMultiModel;

/// A list of block state multiparts
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub struct BlockStateMultiparts {
    /// A list of block state multiparts
    pub multipart: Vec<BlockStateMultipart>,
}

/// A block state multipart
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct BlockStateMultipart {
    /// Conditions for when the multipart should be applied
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<StateConditional>,
    /// The model or list of models to apply
    pub apply: SingleOrMultiModel,
}

/// A conditional state for when a multipart should be applied
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub enum StateConditional {
    /// All conditions must be met
    #[serde(rename = "AND")]
    And(SingleOrMultiCondition),
    /// Any condition must be met
    #[serde(untagged, alias = "OR")]
    Or(SingleOrMultiCondition),
}

/// A single or multiple conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SingleOrMultiCondition {
    /// A single condition
    Single(StateCondition),
    /// A list of conditions
    Multi(Vec<StateCondition>),
}

/// A state condition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct StateCondition {
    /// A condition
    #[serde(flatten)]
    #[reflect(ignore)]
    pub condition: serde_json::Value,
}
