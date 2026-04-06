//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

#[allow(unused_imports, reason = "May be used depending on features")]
use crate::prelude::*;

/// The current state of the physics simulation for an entity.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(CollidingWith, Transform, PreviousTransform))]
#[cfg_attr(feature = "bevy", require(Velocity, Acceleration, OnGround))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct PhysicsState {}

impl PhysicsState {
    /// Create a new [`PhysicsState`].
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }
}
