//! TODO

use core::ops::Deref;

use bevy_ecs::{component::Component, entity::Entity, reflect::ReflectComponent};
use bevy_reflect::Reflect;

/// An [`Entity`](Entity) that is a child of a
/// [`WorldInstance`](crate::bevy::WorldInstance).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash, Component)]
pub struct EntityOfInstance(Entity);

impl EntityOfInstance {
    /// Creates a new [`EntityOfInstance`].
    #[inline]
    #[must_use]
    pub const fn new(entity: Entity) -> Self { Self(entity) }

    /// Get the inner [`Entity`].
    #[inline]
    #[must_use]
    pub const fn entity(&self) -> Entity { self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Deref for EntityOfInstance {
    type Target = Entity;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<Entity> for EntityOfInstance {
    fn from(entity: Entity) -> Self { Self::new(entity) }
}
impl From<EntityOfInstance> for Entity {
    fn from(entity: EntityOfInstance) -> Self { entity.0 }
}
