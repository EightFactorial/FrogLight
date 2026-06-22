use core::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize, std_traits::ReflectDefault};
use glam::{EulerRot, Quat, Vec3A};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `yaw` and `pitch` rotation, in radians.
///
/// Cannot be a [`Quat`] due to how rotations are handled.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", require(PrevRotation))]
pub struct Rotation(Vec3A);

impl Rotation {
    /// The identity rotation.
    pub const IDENTITY: Self = Self(Vec3A::ZERO);

    /// Create a new [`Rotation`] from it's yaw and pitch components.
    #[inline]
    #[must_use]
    pub const fn new(yaw: f32, pitch: f32) -> Self { Self(Vec3A::new(yaw, pitch, 0.0)) }

    /// Create a new [`Rotation`] from a [`Quat`].
    #[inline]
    #[must_use]
    pub fn new_quat(quat: Quat) -> Self {
        let (yaw, pitch, _roll) = quat.to_euler(EulerRot::YXZ);
        Self(Vec3A::new(yaw, pitch, 0.0))
    }

    /// Get the `yaw` component.
    #[inline]
    #[must_use]
    pub const fn yaw(&self) -> f32 { self.0.to_array()[0] }

    /// Get a mutable reference to the `yaw` component.
    #[inline]
    #[must_use]
    pub fn yaw_mut(&mut self) -> &mut f32 { &mut self.0.x }

    /// Get the `pitch` component.
    #[inline]
    #[must_use]
    pub const fn pitch(&self) -> f32 { self.0.to_array()[1] }

    /// Get a mutable reference to the `pitch` component.
    #[inline]
    #[must_use]
    pub fn pitch_mut(&mut self) -> &mut f32 { &mut self.0.y }

    /// Get the underlying [`Vec3A`] of this [`Rotation`].
    ///
    /// # Note
    ///
    /// This is *not* a vector!
    /// It is just being stored in a [`Vec3A`] for performance.
    ///
    /// Stored as `[yaw, pitch, 0.0]`.
    #[inline]
    #[must_use]
    pub const fn as_vec3a(&mut self) -> &mut Vec3A { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Add for Rotation {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) }
}
impl AddAssign for Rotation {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

impl Sub for Rotation {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) }
}
impl SubAssign for Rotation {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
}

// -------------------------------------------------------------------------------------------------

/// The previous tick's [`Rotation`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Serialize, Deserialize))]
pub struct PrevRotation(Rotation);

impl PrevRotation {
    /// Create a new [`PrevRotation`] from it's yaw and pitch components.
    #[inline]
    #[must_use]
    pub const fn new(yaw: f32, pitch: f32) -> Self { Self(Rotation::new(yaw, pitch)) }

    /// Create a new [`PrevRotation`] from an [`Quat`].
    #[inline]
    #[must_use]
    pub fn new_quat(quat: Quat) -> Self { Self(Rotation::new_quat(quat)) }

    /// Create a new [`PrevRotation`] from a [`Rotation`].
    #[inline]
    #[must_use]
    pub const fn new_rot(rotation: Rotation) -> Self { Self(rotation) }

    /// Get the `yaw` component.
    #[inline]
    #[must_use]
    pub const fn yaw(&self) -> f32 { self.0.yaw() }

    /// Get a mutable reference to the `yaw` component.
    #[inline]
    #[must_use]
    pub fn yaw_mut(&mut self) -> &mut f32 { self.0.yaw_mut() }

    /// Get the `pitch` component.
    #[inline]
    #[must_use]
    pub const fn pitch(&self) -> f32 { self.0.pitch() }

    /// Get a mutable reference to the `pitch` component.
    #[inline]
    #[must_use]
    pub fn pitch_mut(&mut self) -> &mut f32 { self.0.pitch_mut() }
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

impl Add for PrevRotation {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) }
}
impl AddAssign for PrevRotation {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

impl Sub for PrevRotation {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) }
}
impl SubAssign for PrevRotation {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
}
