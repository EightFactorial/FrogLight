use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(feature = "bevy")]
use bevy_ecs::reflect::ReflectComponent;
#[cfg(feature = "bevy")]
use bevy_reflect::std_traits::ReflectDefault;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogTest;
use glam::{DVec2, I64Vec2, IVec2, Vec2};

use crate::protocol::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite, ReadError, WriteError};

/// A position in the world, measured in chunks.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, From, Into, Deref, DerefMut, FrogTest,
)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component, bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Default, Component))]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ChunkPosition(#[cfg_attr(feature = "bevy", reflect(ignore))] I64Vec2);

impl ChunkPosition {
    /// All zeros.
    pub const ZERO: Self = Self(I64Vec2::ZERO);

    /// All ones.
    pub const ONE: Self = Self(I64Vec2::ONE);

    /// All negative ones.
    pub const NEG_ONE: Self = Self(I64Vec2::NEG_ONE);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self(I64Vec2::X);
    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self(I64Vec2::Y);

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self(I64Vec2::NEG_X);
    /// A unit vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self(I64Vec2::NEG_Y);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Z];

    /// All `i64::MIN`.
    pub const MIN: Self = Self(I64Vec2::MIN);

    /// All `i64::MAX`.
    pub const MAX: Self = Self(I64Vec2::MAX);

    /// Creates a new [`ChunkPosition`] with the given coordinates.
    #[must_use]
    #[inline]
    pub const fn new(x: i64, z: i64) -> Self { Self(I64Vec2::new(x, z)) }

    /// Creates a new [`ChunkPosition`] with the given coordinates.
    #[must_use]
    #[inline]
    pub const fn new_i32(x: i32, z: i32) -> Self { Self(I64Vec2::new(x as i64, z as i64)) }

    /// Creates a new [`ChunkPosition`] where all coordinates are the same.
    #[must_use]
    #[inline]
    pub const fn splat(v: i64) -> Self { Self(I64Vec2::splat(v)) }

    /// Creates a new [`ChunkPosition`] where all coordinates are the same.
    #[must_use]
    #[inline]
    pub const fn splat_i32(v: i32) -> Self { Self(I64Vec2::splat(v as i64)) }

    /// Inverts all coordinates.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::ChunkPosition;
    ///
    /// assert_eq!(ChunkPosition::ZERO.invert(), ChunkPosition::ZERO);
    /// assert_eq!(ChunkPosition::ONE.invert(), ChunkPosition::NEG_ONE);
    ///
    /// assert_eq!(ChunkPosition::new(1, 2).invert(), ChunkPosition::new(-1, -2));
    /// ```
    #[must_use]
    #[inline]
    pub const fn invert(self) -> Self { Self::new(-self.x(), -self.z()) }

    /// Gets the x-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn x(&self) -> i64 { self.0.x }

    /// Gets the z-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn z(&self) -> i64 { self.0.y }
}

/// Read as i32s and then converted to i64s.
impl FrogRead for ChunkPosition {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        // Swap the X/Z coordinates
        let pos = IVec2::fg_read(buf)?;
        Ok(Self::new(i64::from(pos.y), i64::from(pos.x)))
    }
}

/// Read as variable length i32s and then converted to i64s.
impl FrogVarRead for ChunkPosition {
    fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        // Swap the X/Z coordinates
        let pos = IVec2::fg_var_read(buf)?;
        Ok(Self::new(i64::from(pos.y), i64::from(pos.x)))
    }
}

/// Converted and written as i32s.
impl FrogWrite for ChunkPosition {
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        // Swap the X/Z coordinates
        IVec2::new(i32::try_from(self.y)?, i32::try_from(self.x)?).fg_write(buf)
    }
}

/// Converted and written as variable length i32s.
impl FrogVarWrite for ChunkPosition {
    fn fg_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        // Swap the X/Z coordinates
        IVec2::new(i32::try_from(self.y)?, i32::try_from(self.x)?).fg_var_write(buf)
    }
}

impl Add<ChunkPosition> for ChunkPosition {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output { Self(self.0.add(rhs.0)) }
}

impl AddAssign<ChunkPosition> for ChunkPosition {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.0.add_assign(rhs.0) }
}

impl Sub<ChunkPosition> for ChunkPosition {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { Self(self.0.sub(rhs.0)) }
}

impl SubAssign<ChunkPosition> for ChunkPosition {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.0.sub_assign(rhs.0) }
}

impl Mul<ChunkPosition> for ChunkPosition {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output { Self(self.0.mul(rhs.0)) }
}

impl MulAssign<ChunkPosition> for ChunkPosition {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) { self.0.mul_assign(rhs.0) }
}

impl Mul<i64> for ChunkPosition {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i64) -> Self::Output { Self(self.0.mul(rhs)) }
}

impl MulAssign<i64> for ChunkPosition {
    #[inline]
    fn mul_assign(&mut self, rhs: i64) { self.0.mul_assign(rhs) }
}

impl Div<ChunkPosition> for ChunkPosition {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output { Self(self.0.div(rhs.0)) }
}

impl DivAssign<ChunkPosition> for ChunkPosition {
    #[inline]
    fn div_assign(&mut self, rhs: Self) { self.0.div_assign(rhs.0) }
}

impl Div<i64> for ChunkPosition {
    type Output = Self;
    #[inline]
    fn div(self, rhs: i64) -> Self::Output { Self(self.0.div(rhs)) }
}

impl DivAssign<i64> for ChunkPosition {
    #[inline]
    fn div_assign(&mut self, rhs: i64) { self.0.div_assign(rhs) }
}

impl From<Vec2> for ChunkPosition {
    #[inline]
    fn from(v: Vec2) -> Self { Self(v.as_i64vec2()) }
}

impl From<ChunkPosition> for Vec2 {
    #[inline]
    fn from(v: ChunkPosition) -> Self { v.0.as_vec2() }
}

impl From<DVec2> for ChunkPosition {
    #[inline]
    fn from(v: DVec2) -> Self { Self(v.as_i64vec2()) }
}

impl From<ChunkPosition> for DVec2 {
    #[inline]
    fn from(v: ChunkPosition) -> Self { v.0.as_dvec2() }
}

impl From<IVec2> for ChunkPosition {
    #[inline]
    fn from(v: IVec2) -> Self { Self(v.as_i64vec2()) }
}

impl From<ChunkPosition> for IVec2 {
    #[inline]
    fn from(v: ChunkPosition) -> Self { v.as_ivec2() }
}

impl<T: Into<i64>> From<[T; 2]> for ChunkPosition {
    fn from([first, second]: [T; 2]) -> Self { Self::new(first.into(), second.into()) }
}

impl<T: Into<i64>> From<(T, T)> for ChunkPosition {
    fn from((first, second): (T, T)) -> Self { Self::new(first.into(), second.into()) }
}
