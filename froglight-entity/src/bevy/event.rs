use core::ops::{Deref, DerefMut};

use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

/// An [`Event`] that is emitted when an [`EntityBundle`] is inserted onto an
/// [`Entity`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EntityEvent, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash, Event)]
pub struct EntityBundleEvent(Entity);

impl EntityBundleEvent {
    /// Create a [`EntityBundleEvent`] for the given [`Entity`].
    #[inline]
    #[must_use]
    pub const fn new(entity: Entity) -> Self { Self(entity) }

    /// Get the [`Entity`] associated with this [`EntityBundleEvent`].
    #[inline]
    #[must_use]
    pub const fn entity(self) -> Entity { self.0 }
}

impl Deref for EntityBundleEvent {
    type Target = Entity;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for EntityBundleEvent {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<Entity> for EntityBundleEvent {
    #[inline]
    fn from(entity: Entity) -> Self { Self(entity) }
}
impl From<EntityBundleEvent> for Entity {
    #[inline]
    fn from(event: EntityBundleEvent) -> Self { event.0 }
}
