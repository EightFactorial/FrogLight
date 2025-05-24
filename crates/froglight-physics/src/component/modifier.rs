//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

/// The physics modifiers applied to an entity.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct EntityPhysicsModifiers {}

// -------------------------------------------------------------------------------------------------

/// A modifier applied to an entity to influence its physics.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub struct PhysicsModifier {}
