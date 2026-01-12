//! TODO

use alloc::vec::Vec;
use core::ops::Deref;

use bevy_ecs::{component::Component, entity::Entity, reflect::ReflectComponent};
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

/// A collection of [`Entities`](Entity) owned by a
/// [`WorldInstance`](crate::bevy::WorldInstance).
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Debug, Default, Clone, PartialEq, Hash, Component)]
#[relationship_target(relationship = EntityOf, linked_spawn)]
pub struct WorldEntities {
    entities: Vec<Entity>,
}

/// An [`Entity`](Entity) that is a child of a
/// [`WorldInstance`](crate::bevy::WorldInstance).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash, Component)]
#[relationship(relationship_target = WorldEntities)]
pub struct EntityOf(Entity);

impl Deref for WorldEntities {
    type Target = [Entity];

    fn deref(&self) -> &Self::Target { &self.entities }
}
impl Deref for EntityOf {
    type Target = Entity;

    fn deref(&self) -> &Self::Target { &self.0 }
}

// -------------------------------------------------------------------------------------------------

/// A collection of chunk [`Entities`](Entity) owned by a
/// [`WorldInstance`](crate::bevy::WorldInstance).
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(Debug, Default, Clone, PartialEq, Component)]
#[relationship_target(relationship = ChunkOf, linked_spawn)]
pub struct WorldChunks {
    entities: Vec<Entity>,
}

/// An [`Entity`](Entity) that is a child of a
/// [`WorldInstance`](crate::bevy::WorldInstance).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash, Component)]
#[relationship(relationship_target = WorldChunks)]
pub struct ChunkOf(Entity);

impl Deref for WorldChunks {
    type Target = [Entity];

    fn deref(&self) -> &Self::Target { &self.entities }
}
impl Deref for ChunkOf {
    type Target = Entity;

    fn deref(&self) -> &Self::Target { &self.0 }
}
