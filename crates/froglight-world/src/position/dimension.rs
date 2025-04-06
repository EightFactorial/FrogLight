#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::Identifier;

use super::BlockPos;

/// A position in a specific dimension.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_io::prelude::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
pub struct DimensionPos {
    dim: Identifier,
    pos: BlockPos,
}

impl DimensionPos {
    /// Create a new [`DimensionPos`] with the given
    /// dimension [`Identifier`] and [`BlockPos`].
    #[must_use]
    pub const fn new(dimension: Identifier, position: BlockPos) -> Self {
        Self { dim: dimension, pos: position }
    }

    /// Try to create a new [`DimensionPos`] with the given
    /// dimension [`Identifier`] and [`BlockPos`].
    ///
    /// Returns `None` if the dimension identifier is invalid.
    #[must_use]
    pub fn try_new(dimension: &(impl AsRef<str> + ?Sized), position: BlockPos) -> Option<Self> {
        Some(Self { dim: Identifier::try_new(dimension)?, pos: position })
    }

    /// Get the dimension of this [`DimensionPos`].
    #[inline]
    #[must_use]
    pub const fn dimension(&self) -> &Identifier { &self.dim }

    /// Get the dimension of this [`DimensionPos`] mutably.
    #[inline]
    #[must_use]
    pub const fn dimension_mut(&mut self) -> &mut Identifier { &mut self.dim }

    /// Get the position of this [`DimensionPos`].
    #[inline]
    #[must_use]
    pub const fn position(&self) -> &BlockPos { &self.pos }

    /// Get the position of this [`DimensionPos`] mutably.
    #[inline]
    #[must_use]
    pub const fn position_mut(&mut self) -> &mut BlockPos { &mut self.pos }
}
