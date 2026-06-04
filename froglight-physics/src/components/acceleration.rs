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

/// An acceleration vector.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Add, AddAssign, Sub, SubAssign, Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", require(PrevAcceleration))]
pub struct Acceleration(Vec3A);

impl Acceleration {
    /// All zeros.
    pub const ZERO: Self = Self(Vec3A::ZERO);

    /// Create a new [`Acceleration`] from a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn new(vec: Vec3A) -> Self { Self(vec) }

    /// Create a new [`Acceleration`] from its components.
    #[inline]
    #[must_use]
    pub const fn new_xyz(x: f32, y: f32, z: f32) -> Self { Self::new(Vec3A::new(x, y, z)) }

    /// Convert this [`Acceleration`] into a [`Vec3`].
    #[inline]
    #[must_use]
    pub fn to_vec3(self) -> Vec3 { self.0.to_vec3() }

    /// Convert this [`Acceleration`] into a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn to_vec3a(self) -> Vec3A { self.0 }
}

impl AsRef<Vec3A> for Acceleration {
    #[inline]
    fn as_ref(&self) -> &Vec3A { &self.0 }
}
impl AsMut<Vec3A> for Acceleration {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec3A { &mut self.0 }
}

impl Deref for Acceleration {
    type Target = Vec3A;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Acceleration {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Add for Acceleration {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { Self::new(self.0 + rhs.0) }
}
impl AddAssign for Acceleration {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

impl<T> Add<T> for Acceleration
where
    Vec3A: Add<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output { Self::new(self.0 + rhs) }
}
impl<T> AddAssign<T> for Acceleration
where
    Vec3A: AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) { self.0 += rhs; }
}

impl Sub for Acceleration {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.0 - rhs.0) }
}
impl SubAssign for Acceleration {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
}

impl<T> Sub<T> for Acceleration
where
    Vec3A: Sub<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output { Self::new(self.0 - rhs) }
}
impl<T> SubAssign<T> for Acceleration
where
    Vec3A: SubAssign<T>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) { self.0 -= rhs; }
}

// -------------------------------------------------------------------------------------------------

/// The previous tick's acceleration vector.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Add, AddAssign, Sub, SubAssign, Serialize, Deserialize))]
pub struct PrevAcceleration(Acceleration);

impl PrevAcceleration {
    /// Create a new [`PrevAcceleration`] from a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn new(vec: Vec3A) -> Self { Self(Acceleration::new(vec)) }

    /// Create a new [`PrevAcceleration`] from an [`Acceleration`].
    #[inline]
    #[must_use]
    pub const fn new_accel(accel: Acceleration) -> Self { Self(accel) }

    /// Convert this [`PrevAcceleration`] into an [`Acceleration`].
    #[inline]
    #[must_use]
    pub const fn to_accel(self) -> Acceleration { self.0 }

    /// Convert this [`PrevAcceleration`] into a [`Vec3`].
    #[inline]
    #[must_use]
    pub fn to_vec3(self) -> Vec3 { self.0.to_vec3() }

    /// Convert this [`PrevAcceleration`] into a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn to_vec3a(self) -> Vec3A { self.0.to_vec3a() }
}

impl From<Acceleration> for PrevAcceleration {
    #[inline]
    fn from(accel: Acceleration) -> Self { Self(accel) }
}

impl AsRef<Acceleration> for PrevAcceleration {
    #[inline]
    fn as_ref(&self) -> &Acceleration { &self.0 }
}
impl AsMut<Acceleration> for PrevAcceleration {
    #[inline]
    fn as_mut(&mut self) -> &mut Acceleration { &mut self.0 }
}

impl AsRef<Vec3A> for PrevAcceleration {
    #[inline]
    fn as_ref(&self) -> &Vec3A { self.0.as_ref() }
}
impl AsMut<Vec3A> for PrevAcceleration {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec3A { self.0.as_mut() }
}

impl Deref for PrevAcceleration {
    type Target = Acceleration;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for PrevAcceleration {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Add for PrevAcceleration {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { Self::new_accel(self.0 + rhs.0) }
}
impl AddAssign for PrevAcceleration {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

impl<T> Add<T> for PrevAcceleration
where
    Vec3A: Add<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output { Self::new(self.0.0 + rhs) }
}
impl<T> AddAssign<T> for PrevAcceleration
where
    Vec3A: AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) { self.0 += rhs; }
}

impl Sub for PrevAcceleration {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { Self::new_accel(self.0 - rhs.0) }
}
impl SubAssign for PrevAcceleration {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
}

impl<T> Sub<T> for PrevAcceleration
where
    Vec3A: Sub<T, Output = Vec3A>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output { Self::new(self.0.0 - rhs) }
}
impl<T> SubAssign<T> for PrevAcceleration
where
    Vec3A: SubAssign<T>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) { self.0 -= rhs; }
}
