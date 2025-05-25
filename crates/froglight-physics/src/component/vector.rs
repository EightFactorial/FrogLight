//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::{prelude::*, query::QueryData};
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use glam::Vec3A;

/// A [`Query`] that retrieves the current and previous
/// position, velocity, and acceleration of an entity.
#[cfg(feature = "bevy")]
#[derive(Debug, PartialEq, QueryData)]
pub struct EntityVectors {
    /// The current and previous position of the entity.
    pub position: &'static EntityPosition,
    /// The current and previous velocity of the entity.
    pub velocity: &'static EntityVelocity,
    /// The current and previous acceleration of the entity.
    pub acceleration: &'static EntityAcceleration,
}

/// A mutable [`Query`] that retrieves the current and previous
/// position, velocity, and acceleration of an entity.
#[cfg(feature = "bevy")]
#[derive(Debug, PartialEq, QueryData)]
#[query_data(mutable)]
pub struct EntityVectorsMut {
    /// The current and previous position of the entity.
    pub position: &'static mut EntityPosition,
    /// The current and previous velocity of the entity.
    pub velocity: &'static mut EntityVelocity,
    /// The current and previous acceleration of the entity.
    pub acceleration: &'static mut EntityAcceleration,
}

// -------------------------------------------------------------------------------------------------

/// An entity's current and previous position.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(EntityVelocity, EntityAcceleration))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct EntityPosition(Vec3A, Vec3A);

impl EntityPosition {
    /// Create an [`EntityPosition`] from a [`Vec3A`]
    #[inline]
    #[must_use]
    pub const fn new(velocity: Vec3A) -> Self { Self(velocity, Vec3A::ZERO) }

    /// Get a reference to the entity's current position
    #[inline]
    #[must_use]
    pub const fn current(&self) -> &Vec3A { &self.0 }

    /// Get a mutable reference to the entity's current position
    #[inline]
    #[must_use]
    pub const fn current_mut(&mut self) -> &mut Vec3A { &mut self.0 }

    /// Get a reference to the entity's previous position
    #[inline]
    #[must_use]
    pub const fn previous(&self) -> &Vec3A { &self.1 }

    /// Get a mutable reference to the entity's previous position
    #[inline]
    #[must_use]
    pub const fn previous_mut(&mut self) -> &mut Vec3A { &mut self.1 }

    /// Convert the [`EntityPosition`] into its inner [`Vec3A`]s
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> (Vec3A, Vec3A) { (self.0, self.1) }
}

impl<T: Into<Vec3A>> From<T> for EntityPosition {
    #[inline]
    fn from(value: T) -> Self { Self::new(value.into()) }
}

// -------------------------------------------------------------------------------------------------

/// An entity's current and previous velocity.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(EntityPosition, EntityAcceleration))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct EntityVelocity(Vec3A, Vec3A);

impl EntityVelocity {
    /// Create a [`EntityVelocity`] from a [`Vec3A`]
    #[inline]
    #[must_use]
    pub const fn new(velocity: Vec3A) -> Self { Self(velocity, Vec3A::ZERO) }

    /// Get a reference to the entity's current velocity
    #[inline]
    #[must_use]
    pub const fn current(&self) -> &Vec3A { &self.0 }

    /// Get a mutable reference to the entity's current velocity
    #[inline]
    #[must_use]
    pub const fn current_mut(&mut self) -> &mut Vec3A { &mut self.0 }

    /// Get a reference to the entity's previous velocity
    #[inline]
    #[must_use]
    pub const fn previous(&self) -> &Vec3A { &self.1 }

    /// Get a mutable reference to the entity's previous velocity
    #[inline]
    #[must_use]
    pub const fn previous_mut(&mut self) -> &mut Vec3A { &mut self.1 }

    /// Convert the [`EntityVelocity`] into its inner [`Vec3A`]s
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> (Vec3A, Vec3A) { (self.0, self.1) }
}

impl<T: Into<Vec3A>> From<T> for EntityVelocity {
    #[inline]
    fn from(value: T) -> Self { Self::new(value.into()) }
}

// -------------------------------------------------------------------------------------------------

/// An entity's current acceleration.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(EntityPosition, EntityVelocity))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct EntityAcceleration(Vec3A, Vec3A);

impl EntityAcceleration {
    /// Create a [`EntityAcceleration`] from a [`Vec3A`]
    #[inline]
    #[must_use]
    pub const fn new(acceleration: Vec3A) -> Self { Self(acceleration, Vec3A::ZERO) }

    /// Get a reference to the entity's current acceleration
    #[inline]
    #[must_use]
    pub const fn current(&self) -> &Vec3A { &self.0 }

    /// Get a mutable reference to the entity's current acceleration
    #[inline]
    #[must_use]
    pub const fn current_mut(&mut self) -> &mut Vec3A { &mut self.0 }

    /// Get a reference to the entity's previous acceleration
    #[inline]
    #[must_use]
    pub const fn previous(&self) -> &Vec3A { &self.1 }

    /// Get a mutable reference to the entity's previous acceleration
    #[inline]
    #[must_use]
    pub const fn previous_mut(&mut self) -> &mut Vec3A { &mut self.1 }

    /// Convert the [`EntityAcceleration`] into its inner [`Vec3A`]s
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> (Vec3A, Vec3A) { (self.0, self.1) }
}

impl<T: Into<Vec3A>> From<T> for EntityAcceleration {
    #[inline]
    fn from(value: T) -> Self { Self::new(value.into()) }
}
