//! TODO

use core::ops::Deref;

use bevy_ecs::{entity::Entity, system::SystemInput};

/// Context provided to command systems.
pub struct CommandCtx<T> {
    entity: Entity,
    input: T,
}

impl<T> CommandCtx<T> {
    /// Create a new [`CommandCtx`] with the given entity and input.
    #[inline]
    #[must_use]
    pub const fn new(entity: Entity, input: T) -> Self { Self { entity, input } }

    /// Get the [`Entity`] that is executing this command.
    #[inline]
    #[must_use]
    pub const fn entity(&self) -> Entity { self.entity }

    /// Get a reference to the input for this command.
    #[inline]
    #[must_use]
    pub const fn input(&self) -> &T { &self.input }

    /// Get the input for this command.
    #[inline]
    #[must_use]
    pub fn into_input(self) -> T { self.input }
}

impl<T> SystemInput for CommandCtx<T> {
    type Inner<'i> = (Entity, T);
    type Param<'i> = CommandCtx<T>;

    fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> { CommandCtx::new(this.0, this.1) }
}

impl<T> Deref for CommandCtx<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target { self.input() }
}
