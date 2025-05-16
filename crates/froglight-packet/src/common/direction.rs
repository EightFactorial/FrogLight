//! TODO: Fix enum variant ordering
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "io")]
use froglight_io::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "io", derive(FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Serialize, Deserialize))]
pub enum Direction {
    Up,
    Down,
    North,
    South,
    West,
    East,
}

impl Direction {}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "io", derive(FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Serialize, Deserialize))]
pub enum VerticalDirection {
    Up,
    Down,
}

impl VerticalDirection {}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "io", derive(FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Serialize, Deserialize))]
pub enum HorizontalDirection {
    North,
    South,
    West,
    East,
}

impl HorizontalDirection {}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Serialize, Deserialize))]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "io", derive(FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Serialize, Deserialize))]
pub enum SignedAxis {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl SignedAxis {}
