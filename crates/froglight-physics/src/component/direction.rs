//! TODO

use core::{
    borrow::{Borrow, BorrowMut},
    f32::consts::{FRAC_PI_2, PI},
};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use glam::{Vec2, Vec3};

use crate::table::{EPSILON, cos, sin};

/// The direction an entity is looking in.
///
/// Stored as a [`Vec2`] where `x` is the pitch and `y` is the yaw.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct LookDirection(Vec2);

impl LookDirection {
    /// Looking straight down, positive along the z-axis.
    pub const DOWN: Self = Self(Vec2::new(FRAC_PI_2, 0.0));
    /// Looking straight east, positive along the x-axis.
    pub const EAST: Self = Self(Vec2::new(0.0, -FRAC_PI_2));
    /// Looking straight north, negative along the z-axis.
    pub const NORTH: Self = Self(Vec2::new(0.0, PI));
    /// Looking straight south, positive along the z-axis.
    pub const SOUTH: Self = Self(Vec2::new(0.0, 0.0));
    /// Looking straight up, positive along the z-axis.
    pub const UP: Self = Self(Vec2::new(-FRAC_PI_2, 0.0));
    /// Looking straight west, negative along the x-axis.
    pub const WEST: Self = Self(Vec2::new(0.0, FRAC_PI_2));

    /// Create a [`EntityLookDirection`] from pitch and yaw angles.
    #[inline]
    #[must_use]
    pub const fn new(pitch: f32, yaw: f32) -> Self { Self(Vec2::new(pitch, yaw)) }

    /// Create a [`EntityLookDirection`] from pitch and yaw angles in degrees.
    #[inline]
    #[must_use]
    pub const fn new_degrees(pitch: f32, yaw: f32) -> Self {
        Self::new(pitch.to_radians(), yaw.to_radians())
    }

    /// Get the pitch angle of the look direction.
    ///
    /// # Note
    /// Values will not be clamped and may exceed the range of `[-π/2, π/2]`.
    #[inline]
    #[must_use]
    pub const fn pitch(&self) -> f32 { self.0.x }

    /// Get a mutable reference to the pitch angle of the look direction.
    ///
    /// # Note
    /// Values will not be clamped and may exceed the range of `[-π/2, π/2]`.
    #[inline]
    #[must_use]
    pub const fn pitch_mut(&mut self) -> &mut f32 { &mut self.0.x }

    /// Get the pitch angle of the look direction in degrees.
    ///
    /// # Note
    /// Values will not be clamped and may exceed the range of `[-90, 90]`.
    #[inline]
    #[must_use]
    pub const fn pitch_degrees(&self) -> f32 { self.pitch().to_degrees() }

    /// Get the yaw angle of the look direction.
    ///
    /// # Note
    /// Values will not be clamped and may exceed the range of `[-π, π]`.
    #[inline]
    #[must_use]
    pub const fn yaw(&self) -> f32 { self.0.y }

    /// Get a mutable reference to the yaw angle of the look direction.
    ///
    /// # Note
    /// Values will not be clamped and may exceed the range of `[-π, π]`.
    #[inline]
    #[must_use]
    pub const fn yaw_mut(&mut self) -> &mut f32 { &mut self.0.y }

    /// Get the yaw angle of the look direction in degrees.
    ///
    /// # Note
    /// Values will not be clamped and may exceed the range of `[-180, 180]`.
    #[inline]
    #[must_use]
    pub const fn yaw_degrees(&self) -> f32 { self.yaw().to_degrees() }

    /// Get a reference to the underlying [`Vec2`].
    #[inline]
    #[must_use]
    pub const fn as_vec(&self) -> &Vec2 { &self.0 }

    /// Get a mutable reference to the underlying [`Vec2`].
    #[inline]
    #[must_use]
    pub const fn as_vec_mut(&mut self) -> &mut Vec2 { &mut self.0 }

    /// Create a [`Vec3`] starting at the eyes
    /// and pointing in the direction the entity is looking.
    ///
    /// # Examples
    /// ```
    /// use froglight_physics::prelude::LookDirection;
    /// use glam::Vec3;
    ///
    /// assert_eq!(LookDirection::DOWN.look_vector(), Vec3::new(0.0, -1.0, 0.0));
    /// assert_eq!(LookDirection::UP.look_vector(), Vec3::new(0.0, 1.0, 0.0));
    /// assert_eq!(LookDirection::NORTH.look_vector(), Vec3::new(0.0, 0.0, -1.0));
    /// assert_eq!(LookDirection::SOUTH.look_vector(), Vec3::new(0.0, 0.0, 1.0));
    /// assert_eq!(LookDirection::WEST.look_vector(), Vec3::new(-1.0, 0.0, 0.0));
    /// assert_eq!(LookDirection::EAST.look_vector(), Vec3::new(1.0, 0.0, 0.0));
    /// ```
    #[must_use]
    #[expect(clippy::obfuscated_if_else)]
    pub fn look_vector(&self) -> Vec3 {
        let (pitch, yaw) = self.0.into();
        let (pitch_sin, pitch_cos) = (sin(pitch), cos(pitch));
        let (yaw_sin, yaw_cos) = (sin(-yaw), cos(-yaw));

        let (x, y, z) = (pitch_cos * yaw_sin, -pitch_sin, pitch_cos * yaw_cos);
        Vec3::new(
            x.abs().lt(&EPSILON).then_some(0.0).unwrap_or(x),
            y.abs().lt(&EPSILON).then_some(0.0).unwrap_or(y),
            z.abs().lt(&EPSILON).then_some(0.0).unwrap_or(z),
        )
        .normalize_or_zero()
    }
}

impl<T: Into<Vec2>> From<T> for LookDirection {
    #[inline]
    fn from(value: T) -> Self { Self(value.into()) }
}

impl Borrow<Vec2> for LookDirection {
    #[inline]
    fn borrow(&self) -> &Vec2 { &self.0 }
}
impl BorrowMut<Vec2> for LookDirection {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Vec2 { &mut self.0 }
}

impl AsRef<Vec2> for LookDirection {
    #[inline]
    fn as_ref(&self) -> &Vec2 { &self.0 }
}
impl AsMut<Vec2> for LookDirection {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec2 { &mut self.0 }
}
