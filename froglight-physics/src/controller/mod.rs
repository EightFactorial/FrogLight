//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

#[allow(unused_imports, reason = "WIP")]
use crate::prelude::*;

/// A controller for physics entities.
///
/// Allows for performing inputs and other actions.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(PhysicsState))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct PhysicsController {}

impl PhysicsController {
    /// Create a new [`PhysicsController`].
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }
}
