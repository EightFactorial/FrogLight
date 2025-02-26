use bevy_ecs::{entity::Entity, event::Event};
use bevy_reflect::Reflect;

/// A brigadier command sent by an [`Entity`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event, Reflect)]
#[reflect(Debug, Hash)]
pub struct BrigadierEvent {
    entity: Entity,
    command: String,
}

impl BrigadierEvent {
    /// Create a new [`BrigadierEvent`] with the given [`Entity`] and command.
    #[inline]
    #[must_use]
    pub const fn new(entity: Entity, command: String) -> Self { Self { entity, command } }

    /// Create a new [`BrigadierEvent`] with the given [`Entity`] and command.
    #[inline]
    #[must_use]
    pub fn new_from(entity: Entity, command: impl Into<String>) -> Self {
        Self::new(entity, command.into())
    }

    /// Get the [`Entity`] that triggered the event.
    #[inline]
    #[must_use]
    pub const fn entity(&self) -> Entity { self.entity }

    /// Get the command that was executed.
    #[inline]
    #[must_use]
    pub const fn command(&self) -> &String { &self.command }
}
