use core::ops::{Deref, DerefMut, Mul, MulAssign};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize, std_traits::ReflectDefault};
use glam::Quat;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A rotation matrix.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Reflect, Component))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", require(PrevRotation))]
pub struct Rotation(Quat);

impl Rotation {
    /// The identity rotation.
    pub const IDENTITY: Self = Self(Quat::IDENTITY);

    /// Create a new [`Rotation`] from a [`Quat`].
    #[inline]
    #[must_use]
    pub const fn new(quat: Quat) -> Self { Self(quat) }

    /// Create a new [`Rotation`] from it's components.
    ///
    /// See [`Quat::from_xyzw`] for more details.
    #[inline]
    #[must_use]
    pub const fn new_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self::new(Quat::from_xyzw(x, y, z, w))
    }

    /// Convert this [`Rotation`] into a [`Quat`].
    #[inline]
    #[must_use]
    pub const fn to_quat(self) -> Quat { self.0 }
}

impl AsRef<Quat> for Rotation {
    #[inline]
    fn as_ref(&self) -> &Quat { &self.0 }
}
impl AsMut<Quat> for Rotation {
    #[inline]
    fn as_mut(&mut self) -> &mut Quat { &mut self.0 }
}

impl Deref for Rotation {
    type Target = Quat;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Rotation {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Mul for Rotation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output { Self::new(self.0 * rhs.0) }
}
impl MulAssign for Rotation {
    fn mul_assign(&mut self, rhs: Self) { self.0 *= rhs.0; }
}

impl<T> Mul<T> for Rotation
where
    Quat: Mul<T, Output = Quat>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output { Self::new(self.0 * rhs) }
}
impl<T> MulAssign<T> for Rotation
where
    Quat: MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: T) { self.0 *= rhs; }
}

// -------------------------------------------------------------------------------------------------

/// The previous tick's [`Rotation`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Reflect, Component))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Serialize, Deserialize))]
pub struct PrevRotation(Rotation);

impl PrevRotation {
    /// Create a new [`PrevRotation`] from an [`Quat`].
    #[inline]
    #[must_use]
    pub const fn new(quat: Quat) -> Self { Self(Rotation::new(quat)) }

    /// Create a new [`PrevRotation`] from it's components.
    ///
    /// See [`Quat::from_xyzw`] for more details.
    #[inline]
    #[must_use]
    pub const fn new_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self::new(Quat::from_xyzw(x, y, z, w))
    }

    /// Create a new [`PrevRotation`] from a [`Rotation`].
    #[inline]
    #[must_use]
    pub const fn new_rot(rotation: Rotation) -> Self { Self(rotation) }

    /// Convert this [`Rotation`] into a [`Quat`].
    #[inline]
    #[must_use]
    pub const fn to_quat(self) -> Quat { self.0.0 }
}

impl From<Rotation> for PrevRotation {
    #[inline]
    fn from(rotation: Rotation) -> Self { Self(rotation) }
}

impl AsRef<Rotation> for PrevRotation {
    #[inline]
    fn as_ref(&self) -> &Rotation { &self.0 }
}
impl AsMut<Rotation> for PrevRotation {
    #[inline]
    fn as_mut(&mut self) -> &mut Rotation { &mut self.0 }
}

impl AsRef<Quat> for PrevRotation {
    #[inline]
    fn as_ref(&self) -> &Quat { self.0.as_ref() }
}
impl AsMut<Quat> for PrevRotation {
    #[inline]
    fn as_mut(&mut self) -> &mut Quat { self.0.as_mut() }
}

impl Deref for PrevRotation {
    type Target = Rotation;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for PrevRotation {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Mul for PrevRotation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output { Self::new(self.0.0 * rhs.0.0) }
}
impl MulAssign for PrevRotation {
    fn mul_assign(&mut self, rhs: Self) { self.0 *= rhs.0; }
}

impl<T> Mul<T> for PrevRotation
where
    Quat: Mul<T, Output = Quat>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output { Self::new_rot(self.0 * rhs) }
}
impl<T> MulAssign<T> for PrevRotation
where
    Quat: MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: T) { self.0 *= rhs; }
}
