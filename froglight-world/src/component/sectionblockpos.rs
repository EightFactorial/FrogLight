#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{SECTION_HEIGHT, SECTION_LENGTH, SECTION_WIDTH};

/// A block's position within a section.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct SectionBlockPos(u16);

impl SectionBlockPos {
    /// Create a new [`SectionBlockPos`] from the given coordinates.
    #[inline]
    #[must_use]
    pub const fn new_index(index: u16) -> Self { Self(index) }

    /// Create a new [`SectionBlockPos`] from the given x, y, and z coordinates.
    #[must_use]
    pub const fn new_xyz(x: u8, y: u8, z: u8) -> Self {
        let x = x.rem_euclid(SECTION_LENGTH) as u16;
        let y = y.rem_euclid(SECTION_HEIGHT) as u16;
        let z = z.rem_euclid(SECTION_WIDTH) as u16;
        Self((y << 8) | (z << 4) | x)
    }

    /// Get the index of this [`SectionBlockPos`].
    #[inline]
    #[must_use]
    pub const fn index(&self) -> u16 { self.0 }

    /// Get the x coordinate of this [`SectionBlockPos`].
    #[must_use]
    pub const fn x(&self) -> u8 { (self.0 & 0xF) as u8 }

    /// Get the y coordinate of this [`SectionBlockPos`].
    #[must_use]
    pub const fn y(&self) -> u8 { ((self.0 >> 8) & 0xF) as u8 }

    /// Get the z coordinate of this [`SectionBlockPos`].
    #[must_use]
    pub const fn z(&self) -> u8 { ((self.0 >> 4) & 0xF) as u8 }
}
