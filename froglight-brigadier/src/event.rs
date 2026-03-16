//! TODO

use alloc::borrow::Cow;

use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

/// An [`Event`] that executes a game command as an [`Entity`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, EntityEvent)]
#[reflect(Debug, Clone, PartialEq, Hash, Event)]
pub struct GameCommandEvent {
    entity: Entity,
    command: Cow<'static, str>,
}

impl GameCommandEvent {
    /// Create a new [`GameCommandEvent`].
    #[inline]
    #[must_use]
    pub fn new<T: Into<Cow<'static, str>>>(entity: Entity, command: T) -> Self {
        Self { entity, command: command.into() }
    }

    /// Get the [`Entity`] that will execute the command.
    #[inline]
    #[must_use]
    pub fn entity(&self) -> Entity { self.entity }

    /// Get the command to be executed.
    #[inline]
    #[must_use]
    pub fn command(&self) -> &str { &self.command }
}
