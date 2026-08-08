//! TODO

use core::ops::{Deref, DerefMut};

use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

use crate::prelude::TickTimer;

/// An [`EntityEvent`] that is sent when a [`TickTimer`] ticks.
///
/// Used for [`Observer`]s.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EntityEvent, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash, Event)]
pub struct Ticked(pub Entity);

impl Ticked {
    /// Create a new [`Ticked`] for the given [`Entity`].
    #[inline]
    #[must_use]
    pub const fn new(entity: Entity) -> Self { Self(entity) }

    /// Get the [`Entity`] associated with this [`Ticked`].
    #[inline]
    #[must_use]
    pub const fn entity(self) -> Entity { self.0 }
}

impl Ticked {
    /// A [`System`] that triggers a [`Ticked`] event for each [`TickTimer`].
    pub fn trigger_ticked(query: Query<Entity, With<TickTimer>>, mut commands: Commands) {
        for entity in query {
            commands.trigger(Ticked::new(entity));
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Deref for Ticked {
    type Target = Entity;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Ticked {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<Entity> for Ticked {
    fn from(entity: Entity) -> Self { Self(entity) }
}
impl From<Ticked> for Entity {
    fn from(ticked: Ticked) -> Self { ticked.0 }
}
