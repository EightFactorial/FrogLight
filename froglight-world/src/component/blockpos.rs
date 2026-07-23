#![allow(
    clippy::unsafe_derive_deserialize,
    reason = "Triggered by deriving `Facet` and `Deserialize`"
)]

use core::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
#[cfg(feature = "facet")]
use froglight_facet::facet::{WithFnAttr, prelude::*};
use glam::IVec3;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{CHUNK_LENGTH, CHUNK_WIDTH, component::ChunkBlockPos, prelude::ChunkPos};

/// A block's position in the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet), facet(opaque))]
#[cfg_attr(feature = "facet", facet(mc::with = BlockPos::WITH))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct BlockPos(IVec3);

impl BlockPos {
    /// A [`BlockPos`] with all coordinates set to `0`.
    pub const ZERO: Self = Self(IVec3::ZERO);

    /// Create a new [`BlockPos`] from the given coordinates.
    #[inline]
    #[must_use]
    pub const fn new(coords: IVec3) -> Self { Self(coords) }

    /// Create a new [`BlockPos`] from the given x, y, and z coordinates.
    #[inline]
    #[must_use]
    pub const fn new_xyz(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }

    /// Create a new [`BlockPos`] with all coordinates set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: i32) -> Self { Self(IVec3::splat(v)) }

    /// Get the x coordinate of this [`BlockPos`].
    #[inline]
    #[must_use]
    pub const fn x(&self) -> i32 { self.0.x }

    /// Get the y coordinate of this [`BlockPos`].
    #[inline]
    #[must_use]
    pub const fn y(&self) -> i32 { self.0.y }

    /// Get the z coordinate of this [`BlockPos`].
    #[inline]
    #[must_use]
    pub const fn z(&self) -> i32 { self.0.z }

    /// Get the coordinates of this [`BlockPos`] as an [`IVec3`].
    #[inline]
    #[must_use]
    pub const fn as_ivec3(&self) -> IVec3 { self.0 }

    /// Create a new [`BlockPos`] from the given [`ChunkBlockPos`],
    /// [`ChunkPos`], and vertical offset.
    #[must_use]
    pub const fn from_chunk_blockpos(block: ChunkBlockPos, chunk: ChunkPos, offset: i32) -> Self {
        Self::new(IVec3::new(
            block.x() as i32 + (chunk.x() * CHUNK_LENGTH as i32),
            block.y() as i32 + offset,
            block.z() as i32 + (chunk.z() * CHUNK_WIDTH as i32),
        ))
    }

    /// Create a [`ChunkPos`] from this [`BlockPos`].
    #[must_use]
    pub const fn into_chunk_pos(self) -> ChunkPos {
        ChunkPos::new_xz(self.x() / CHUNK_LENGTH as i32, self.z() / CHUNK_WIDTH as i32)
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "facet")]
#[expect(clippy::cast_sign_loss, reason = "Expected behavior")]
#[expect(clippy::cast_possible_wrap, reason = "Expected behavior")]
impl FacetTemplate for BlockPos {
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError> {
        let value = item.get::<Self>()?;
        for val in value.as_ivec3().to_array() {
            encode_u32_into(val as u32, writer)?;
        }

        Ok(())
    }

    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        let [mut x, mut y, mut z] = [0; 3];
        for val in [&mut x, &mut y, &mut z] {
            *val = decode_u32_from(reader)? as i32;
        }

        item.set(Self::new_xyz(x, y, z))
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "facet")]
impl BlockPos {
    /// A [`WithFnAttr`] for serializing and deserializing packed
    /// [`BlockPos`].
    ///
    /// See [`FacetTemplate`] for more information.
    pub const WITH_PACKED: WithFnAttr = WithFnAttr::template::<Packed>();
}

#[cfg(feature = "facet")]
struct Packed;

#[rustfmt::skip]
#[cfg(feature = "facet")]
impl Packed {
    // <3 Azalea

    const PACKED_X_LENGTH: u64 = 1 + 25; // minecraft does something a bit more complicated to get this 25
    const PACKED_Z_LENGTH: u64 = Self::PACKED_X_LENGTH;
    const PACKED_Y_LENGTH: u64 = 64 - Self::PACKED_X_LENGTH - Self::PACKED_Z_LENGTH;
    const PACKED_X_MASK: u64 = (1 << Self::PACKED_X_LENGTH) - 1;
    const PACKED_Y_MASK: u64 = (1 << Self::PACKED_Y_LENGTH) - 1;
    const PACKED_Z_MASK: u64 = (1 << Self::PACKED_Z_LENGTH) - 1;
    const Z_OFFSET: u64 = Self::PACKED_Y_LENGTH;
    const X_OFFSET: u64 = Self::PACKED_Y_LENGTH + Self::PACKED_Z_LENGTH;
}

#[rustfmt::skip]
#[cfg(feature = "facet")]
#[expect(clippy::cast_sign_loss, reason = "Expected")]
impl FacetTemplate for Packed {
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError> {
        let item = item.get::<BlockPos>()?;

        let mut val: u64 = 0;
        val |= ((item.x() as u64) & Self::PACKED_X_MASK) << Self::X_OFFSET;
        val |= (item.y() as u64) & Self::PACKED_Y_MASK;
        val |= ((item.z() as u64) & Self::PACKED_Z_MASK) << Self::Z_OFFSET;

        writer.write_bytes(&val.to_be_bytes())
    }

    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        let val = i64::from_be_bytes(*reader.read_array()?);
        let x = (val << (64 - Self::X_OFFSET - Self::PACKED_X_LENGTH) >> (64 - Self::PACKED_X_LENGTH)) as i32;
        let y = (val << (64 - Self::PACKED_Y_LENGTH) >> (64 - Self::PACKED_Y_LENGTH)) as i32;
        let z = (val << (64 - Self::Z_OFFSET - Self::PACKED_Z_LENGTH) >> (64 - Self::PACKED_Z_LENGTH)) as i32;

        item.set(BlockPos::new_xyz(x, y, z))
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: Into<IVec3>> From<T> for BlockPos {
    #[inline]
    fn from(value: T) -> Self { BlockPos::new(value.into()) }
}

impl fmt::Display for BlockPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        core::write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}

impl Add<BlockPos> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn add(self, rhs: BlockPos) -> Self::Output { BlockPos::new(self.0 + rhs.0) }
}
impl Add<[i32; 3]> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn add(self, rhs: [i32; 3]) -> Self::Output { BlockPos::new(self.0 + IVec3::from(rhs)) }
}

impl Sub<BlockPos> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn sub(self, rhs: BlockPos) -> Self::Output { BlockPos::new(self.0 - rhs.0) }
}
impl Sub<[i32; 3]> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn sub(self, rhs: [i32; 3]) -> Self::Output { BlockPos::new(self.0 - IVec3::from(rhs)) }
}

impl Mul<BlockPos> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn mul(self, rhs: BlockPos) -> Self::Output { BlockPos::new(self.0 * rhs.0) }
}
impl Mul<[i32; 3]> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn mul(self, rhs: [i32; 3]) -> Self::Output { BlockPos::new(self.0 * IVec3::from(rhs)) }
}
impl Mul<i32> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output { BlockPos::new(self.0 * rhs) }
}

impl Div<BlockPos> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn div(self, rhs: BlockPos) -> Self::Output { BlockPos::new(self.0 / rhs.0) }
}
impl Div<[i32; 3]> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn div(self, rhs: [i32; 3]) -> Self::Output { BlockPos::new(self.0 / IVec3::from(rhs)) }
}
impl Div<i32> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn div(self, rhs: i32) -> Self::Output { BlockPos::new(self.0 / rhs) }
}
