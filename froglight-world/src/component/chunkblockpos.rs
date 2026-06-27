#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use glam::U8Vec2;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    CHUNK_LENGTH, CHUNK_WIDTH, SECTION_HEIGHT, component::SectionBlockPos, prelude::BlockPos,
};

/// A block's position within a chunk.
///
/// This position is absolute, and does not take into account the chunk's
/// vertical offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ChunkBlockPos(U8Vec2, u16);

impl ChunkBlockPos {
    /// Create a new [`ChunkBlockPos`] from the given x, y, and z coordinates.
    #[must_use]
    pub const fn new_xyz(x: u8, y: u16, z: u8) -> Self {
        Self(U8Vec2::new(x.rem_euclid(CHUNK_LENGTH), z.rem_euclid(CHUNK_WIDTH)), y)
    }

    /// Get the x coordinate of this [`ChunkBlockPos`].
    #[inline]
    #[must_use]
    pub const fn x(&self) -> u8 { self.0.x }

    /// Get the y coordinate of this [`ChunkBlockPos`].
    #[inline]
    #[must_use]
    pub const fn y(&self) -> u16 { self.1 }

    /// Get the z coordinate of this [`ChunkBlockPos`].
    #[inline]
    #[must_use]
    pub const fn z(&self) -> u8 { self.0.y }

    /// Get the index of the section that contains this block.
    #[must_use]
    pub const fn as_section_index(&self) -> usize { self.y() as usize / SECTION_HEIGHT as usize }

    /// Get the [`SectionBlockPos`] of this block.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "Height will never be that large")]
    pub const fn as_section_blockpos(&self) -> SectionBlockPos {
        SectionBlockPos::new_xyz(self.x(), (self.y() % SECTION_HEIGHT as u16) as u8, self.z())
    }

    /// Try to create a new [`ChunkBlockPos`] from the given [`BlockPos`] and
    /// vertical chunk offset.
    ///
    /// Returns `None` if the resulting Y coordinate is negative or exceeds
    /// `u16::MAX`.
    #[must_use]
    pub const fn try_from_blockpos(position: BlockPos, chunk_offset: i32) -> Option<Self> {
        let Some(absolute_y) = position.y().checked_sub(chunk_offset) else { return None };

        if absolute_y.is_negative() || absolute_y > u16::MAX as i32 {
            None
        } else {
            #[expect(clippy::cast_sign_loss, reason = "Verified within bounds above")]
            #[expect(clippy::cast_possible_truncation, reason = "Verified within bounds above")]
            Some(Self(
                U8Vec2::new(
                    position.x().rem_euclid(CHUNK_LENGTH as i32) as u8,
                    position.z().rem_euclid(CHUNK_WIDTH as i32) as u8,
                ),
                absolute_y as u16,
            ))
        }
    }

    /// Try to create a new [`ChunkBlockPos`] from the given
    /// [`SectionBlockPos`] and section index.
    ///
    /// Returns `None` if the resulting Y coordinate exceeds `u16::MAX`.
    #[must_use]
    pub const fn try_from_sectionpos(
        position: SectionBlockPos,
        section_index: usize,
    ) -> Option<Self> {
        let Some(total_height) =
            (position.y() as usize).checked_add(section_index * SECTION_HEIGHT as usize)
        else {
            return None;
        };

        if total_height > u16::MAX as usize {
            None
        } else {
            #[expect(clippy::cast_possible_truncation, reason = "Verified within bounds above")]
            Some(Self(
                U8Vec2::new(position.x(), position.z()),
                position.y() as u16 + total_height as u16,
            ))
        }
    }
}
