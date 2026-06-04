use core::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{
    Reflect, ReflectDeserialize, ReflectSerialize,
    std_traits::{ReflectAdd, ReflectAddAssign, ReflectDefault, ReflectSub, ReflectSubAssign},
};
use glam::{Vec3, Vec3A};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy")]
use crate::components::Rotation;

/// A position.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Add, AddAssign, Sub, SubAssign, Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", require(Rotation, PrevPosition))]
pub struct Position(Vec3A);

impl Position {
    /// All zeros.
    pub const ZERO: Self = Self(Vec3A::ZERO);

    /// Create a new [`Position`] from a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn new(vec: Vec3A) -> Self { Self(vec) }

    /// Create a new [`Position`] from its components.
    #[inline]
    #[must_use]
    pub const fn new_xyz(x: f32, y: f32, z: f32) -> Self { Self::new(Vec3A::new(x, y, z)) }

    /// Convert this [`Position`] into a [`Vec3`].
    #[inline]
    #[must_use]
    pub fn to_vec3(self) -> Vec3 { self.0.to_vec3() }

    /// Convert this [`Position`] into a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn to_vec3a(self) -> Vec3A { self.0 }
}

impl AsRef<Vec3A> for Position {
    #[inline]
    fn as_ref(&self) -> &Vec3A { &self.0 }
}
impl AsMut<Vec3A> for Position {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec3A { &mut self.0 }
}

impl Deref for Position {
    type Target = Vec3A;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Position {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Add for Position {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { Self::new(self.0 + rhs.0) }
}
impl AddAssign for Position {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

impl<T> Add<T> for Position
where
    Vec3A: Add<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output { Self::new(self.0 + rhs) }
}
impl<T> AddAssign<T> for Position
where
    Vec3A: AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) { self.0 += rhs; }
}

impl Sub for Position {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.0 - rhs.0) }
}
impl SubAssign for Position {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
}

impl<T> Sub<T> for Position
where
    Vec3A: Sub<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output { Self::new(self.0 - rhs) }
}
impl<T> SubAssign<T> for Position
where
    Vec3A: SubAssign<T>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) { self.0 -= rhs; }
}

// -------------------------------------------------------------------------------------------------

/// The previous tick's [`Position`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Add, AddAssign, Sub, SubAssign, Serialize, Deserialize))]
pub struct PrevPosition(Position);

impl PrevPosition {
    /// Create a new [`PrevPosition`] from a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn new(vec: Vec3A) -> Self { Self(Position::new(vec)) }

    /// Create a new [`PrevPosition`] from an [`Position`].
    #[inline]
    #[must_use]
    pub const fn new_pos(accel: Position) -> Self { Self(accel) }

    /// Convert this [`PrevPosition`] into an [`Position`].
    #[inline]
    #[must_use]
    pub const fn to_accel(self) -> Position { self.0 }

    /// Convert this [`PrevPosition`] into a [`Vec3`].
    #[inline]
    #[must_use]
    pub fn to_vec3(self) -> Vec3 { self.0.to_vec3() }

    /// Convert this [`PrevPosition`] into a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn to_vec3a(self) -> Vec3A { self.0.to_vec3a() }
}

impl From<Position> for PrevPosition {
    #[inline]
    fn from(accel: Position) -> Self { Self(accel) }
}

impl AsRef<Position> for PrevPosition {
    #[inline]
    fn as_ref(&self) -> &Position { &self.0 }
}
impl AsMut<Position> for PrevPosition {
    #[inline]
    fn as_mut(&mut self) -> &mut Position { &mut self.0 }
}

impl AsRef<Vec3A> for PrevPosition {
    #[inline]
    fn as_ref(&self) -> &Vec3A { self.0.as_ref() }
}
impl AsMut<Vec3A> for PrevPosition {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec3A { self.0.as_mut() }
}

impl Deref for PrevPosition {
    type Target = Position;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for PrevPosition {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Add for PrevPosition {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { Self::new_pos(self.0 + rhs.0) }
}
impl AddAssign for PrevPosition {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

impl<T> Add<T> for PrevPosition
where
    Vec3A: Add<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output { Self::new(self.0.0 + rhs) }
}
impl<T> AddAssign<T> for PrevPosition
where
    Vec3A: AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) { self.0 += rhs; }
}

impl Sub for PrevPosition {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { Self::new_pos(self.0 - rhs.0) }
}
impl SubAssign for PrevPosition {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
}

impl<T> Sub<T> for PrevPosition
where
    Vec3A: Sub<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output { Self::new(self.0.0 - rhs) }
}
impl<T> SubAssign<T> for PrevPosition
where
    Vec3A: SubAssign<T>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) { self.0 -= rhs; }
}
