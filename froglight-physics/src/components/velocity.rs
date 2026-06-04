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
use crate::components::Acceleration;

/// A velocity vector.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Add, AddAssign, Sub, SubAssign, Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", require(Acceleration, PrevVelocity))]
pub struct Velocity(Vec3A);

impl Velocity {
    /// All zeros.
    pub const ZERO: Self = Self(Vec3A::ZERO);

    /// Create a new [`Velocity`] from a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn new(vec: Vec3A) -> Self { Self(vec) }

    /// Create a new [`Velocity`] from its components.
    #[inline]
    #[must_use]
    pub const fn new_xyz(x: f32, y: f32, z: f32) -> Self { Self::new(Vec3A::new(x, y, z)) }

    /// Convert this [`Velocity`] into a [`Vec3`].
    #[inline]
    #[must_use]
    pub fn to_vec3(self) -> Vec3 { self.0.to_vec3() }

    /// Convert this [`Velocity`] into a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn to_vec3a(self) -> Vec3A { self.0 }
}

impl AsRef<Vec3A> for Velocity {
    #[inline]
    fn as_ref(&self) -> &Vec3A { &self.0 }
}
impl AsMut<Vec3A> for Velocity {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec3A { &mut self.0 }
}

impl Deref for Velocity {
    type Target = Vec3A;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Velocity {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Add for Velocity {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { Self::new(self.0 + rhs.0) }
}
impl AddAssign for Velocity {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

impl<T> Add<T> for Velocity
where
    Vec3A: Add<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output { Self::new(self.0 + rhs) }
}
impl<T> AddAssign<T> for Velocity
where
    Vec3A: AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) { self.0 += rhs; }
}

impl Sub for Velocity {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.0 - rhs.0) }
}
impl SubAssign for Velocity {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
}

impl<T> Sub<T> for Velocity
where
    Vec3A: Sub<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output { Self::new(self.0 - rhs) }
}
impl<T> SubAssign<T> for Velocity
where
    Vec3A: SubAssign<T>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) { self.0 -= rhs; }
}

// -------------------------------------------------------------------------------------------------

/// The previous tick's [`Velocity`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Add, AddAssign, Sub, SubAssign, Serialize, Deserialize))]
pub struct PrevVelocity(Velocity);

impl PrevVelocity {
    /// Create a new [`PrevVelocity`] from a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn new(vec: Vec3A) -> Self { Self(Velocity::new(vec)) }

    /// Create a new [`PrevVelocity`] from an [`Velocity`].
    #[inline]
    #[must_use]
    pub const fn new_vel(accel: Velocity) -> Self { Self(accel) }

    /// Convert this [`PrevVelocity`] into an [`Velocity`].
    #[inline]
    #[must_use]
    pub const fn to_accel(self) -> Velocity { self.0 }

    /// Convert this [`PrevVelocity`] into a [`Vec3`].
    #[inline]
    #[must_use]
    pub fn to_vec3(self) -> Vec3 { self.0.to_vec3() }

    /// Convert this [`PrevVelocity`] into a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn to_vec3a(self) -> Vec3A { self.0.to_vec3a() }
}

impl From<Velocity> for PrevVelocity {
    #[inline]
    fn from(velocity: Velocity) -> Self { Self(velocity) }
}

impl AsRef<Velocity> for PrevVelocity {
    #[inline]
    fn as_ref(&self) -> &Velocity { &self.0 }
}
impl AsMut<Velocity> for PrevVelocity {
    #[inline]
    fn as_mut(&mut self) -> &mut Velocity { &mut self.0 }
}

impl AsRef<Vec3A> for PrevVelocity {
    #[inline]
    fn as_ref(&self) -> &Vec3A { self.0.as_ref() }
}
impl AsMut<Vec3A> for PrevVelocity {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec3A { self.0.as_mut() }
}

impl Deref for PrevVelocity {
    type Target = Velocity;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for PrevVelocity {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Add for PrevVelocity {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { Self::new_vel(self.0 + rhs.0) }
}
impl AddAssign for PrevVelocity {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

impl<T> Add<T> for PrevVelocity
where
    Vec3A: Add<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output { Self::new(self.0.0 + rhs) }
}
impl<T> AddAssign<T> for PrevVelocity
where
    Vec3A: AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) { self.0 += rhs; }
}

impl Sub for PrevVelocity {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { Self::new_vel(self.0 - rhs.0) }
}
impl SubAssign for PrevVelocity {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
}

impl<T> Sub<T> for PrevVelocity
where
    Vec3A: Sub<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output { Self::new(self.0.0 - rhs) }
}
impl<T> SubAssign<T> for PrevVelocity
where
    Vec3A: SubAssign<T>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) { self.0 -= rhs; }
}
