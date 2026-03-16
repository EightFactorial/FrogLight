//! TODO

use core::ops::Deref;

use bevy_ecs::{entity::Entity, system::SystemInput};

/// Context provided to game command systems.
pub struct GameCommandCtx<T> {
    entity: Entity,
    input: T,
}

impl<T> GameCommandCtx<T> {
    /// Create a new [`GameCommandCtx`] with the given entity and input.
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

impl<T> SystemInput for GameCommandCtx<T> {
    type Inner<'i> = (Entity, T);
    type Param<'i> = GameCommandCtx<T>;

    fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> { GameCommandCtx::new(this.0, this.1) }
}

impl<T> Deref for GameCommandCtx<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target { self.input() }
}
