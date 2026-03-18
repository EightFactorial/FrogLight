//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

#[allow(unused_imports, reason = "WIP")]
use crate::prelude::*;

/// The current state of the physics simulation for an entity.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(Transform, PreviousTransform))]
#[cfg_attr(feature = "bevy", require(Velocity, Acceleration, OnGround))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct PhysicsState {}

impl PhysicsState {
    /// Create a new [`PhysicsState`].
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }
}
