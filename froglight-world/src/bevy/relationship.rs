//! TODO

use core::ops::Deref;

use bevy_ecs::{component::Component, entity::Entity, reflect::ReflectComponent};
use bevy_reflect::Reflect;

/// An [`Entity`] that is a child of a
/// [`WorldInstance`](crate::bevy::WorldInstance).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash, Component)]
pub struct ChunkOfInstance(Entity);

impl ChunkOfInstance {
    /// Creates a new [`ChunkOfInstance`].
    #[inline]
    #[must_use]
    pub const fn new(entity: Entity) -> Self { Self(entity) }

    /// Get the inner [`Entity`].
    #[inline]
    #[must_use]
    pub const fn entity(&self) -> Entity { self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Deref for ChunkOfInstance {
    type Target = Entity;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<Entity> for ChunkOfInstance {
    fn from(entity: Entity) -> Self { Self::new(entity) }
}
impl From<ChunkOfInstance> for Entity {
    fn from(entity: ChunkOfInstance) -> Self { entity.0 }
}
