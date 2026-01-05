#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use froglight_common::prelude::Identifier;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::prelude::BlockPos;

/// A block's position and dimension.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct DimensionPos {
    /// The dimension's identifier.
    pub dimension: Identifier<'static>,
    /// The block's position within the world.
    pub position: BlockPos,
}
