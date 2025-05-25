//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::{prelude::*, query::QueryData};
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use glam::Vec3;

/// A [`Query`] that contains the
/// position, velocity, and acceleration of an entity.
#[cfg(feature = "bevy")]
#[derive(Debug, PartialEq, QueryData)]
#[query_data(mutable)]
pub struct CurrentVectors {
    /// The current position of the entity.
    pub position: &'static mut CurrentPosition,
    /// The current velocity of the entity.
    pub velocity: &'static mut CurrentVelocity,
    /// The current acceleration of the entity.
    pub acceleration: &'static mut CurrentAcceleration,
}

/// A [`Query`] that contains the previous
/// position, velocity, and acceleration of an entity.
#[cfg(feature = "bevy")]
#[derive(Debug, PartialEq, QueryData)]
#[query_data(mutable)]
pub struct PreviousVectors {
    /// The previous position of the entity.
    pub position: &'static mut PreviousPosition,
    /// The previous velocity of the entity.
    pub velocity: &'static mut PreviousVelocity,
    /// The previous acceleration of the entity.
    pub acceleration: &'static mut PreviousAcceleration,
}

// -------------------------------------------------------------------------------------------------

/// An entity's current position.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(CurrentVelocity, CurrentAcceleration, PreviousPosition))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct CurrentPosition(Vec3);

impl CurrentPosition {
    /// Create a [`CurrentPosition`] from a [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn new(velocity: Vec3) -> Self { Self(velocity) }

    /// Convert the [`CurrentPosition`] into its inner [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> Vec3 { self.0 }
}

impl<T: Into<Vec3>> From<T> for CurrentPosition {
    #[inline]
    fn from(value: T) -> Self { Self(value.into()) }
}

// -------------------------------------------------------------------------------------------------

/// An entity's position during the previous tick.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect), require(CurrentPosition))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct PreviousPosition(Vec3);

impl PreviousPosition {
    /// Create a [`PreviousPosition`] from a [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn new(velocity: Vec3) -> Self { Self(velocity) }

    /// Convert the [`PreviousPosition`] into its inner [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> Vec3 { self.0 }
}

impl<T: Into<Vec3>> From<T> for PreviousPosition {
    #[inline]
    fn from(value: T) -> Self { Self(value.into()) }
}

// -------------------------------------------------------------------------------------------------

/// An entity's current velocity.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(CurrentPosition, CurrentAcceleration, PreviousVelocity))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct CurrentVelocity(Vec3);

impl CurrentVelocity {
    /// Create a [`CurrentVelocity`] from a [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn new(velocity: Vec3) -> Self { Self(velocity) }

    /// Convert the [`CurrentVelocity`] into its inner [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> Vec3 { self.0 }
}

impl<T: Into<Vec3>> From<T> for CurrentVelocity {
    #[inline]
    fn from(value: T) -> Self { Self(value.into()) }
}

// -------------------------------------------------------------------------------------------------

/// An entity's velocity during the previous tick.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect), require(CurrentVelocity))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct PreviousVelocity(Vec3);

impl PreviousVelocity {
    /// Create a [`PreviousVelocity`] from a [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn new(velocity: Vec3) -> Self { Self(velocity) }

    /// Convert the [`PreviousVelocity`] into its inner [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> Vec3 { self.0 }
}

impl<T: Into<Vec3>> From<T> for PreviousVelocity {
    #[inline]
    fn from(value: T) -> Self { Self(value.into()) }
}

// -------------------------------------------------------------------------------------------------

/// An entity's current acceleration.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(CurrentPosition, CurrentVelocity, PreviousAcceleration))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct CurrentAcceleration(Vec3);

impl CurrentAcceleration {
    /// Create a [`CurrentAcceleration`] from a [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn new(acceleration: Vec3) -> Self { Self(acceleration) }

    /// Convert the [`CurrentAcceleration`] into its inner [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> Vec3 { self.0 }
}

impl<T: Into<Vec3>> From<T> for CurrentAcceleration {
    #[inline]
    fn from(value: T) -> Self { Self(value.into()) }
}

// -------------------------------------------------------------------------------------------------

/// An entity's current acceleration.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect), require(CurrentAcceleration))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct PreviousAcceleration(Vec3);

impl PreviousAcceleration {
    /// Create a [`PreviousAcceleration`] from a [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn new(acceleration: Vec3) -> Self { Self(acceleration) }

    /// Convert the [`PreviousAcceleration`] into its inner [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> Vec3 { self.0 }
}

impl<T: Into<Vec3>> From<T> for PreviousAcceleration {
    #[inline]
    fn from(value: T) -> Self { Self(value.into()) }
}
