use core::ops::{Deref, DerefMut};

use bevy_ecs::{
    entity::{EntityHashMap, EntityHashSet},
    lifecycle::HookContext,
    prelude::*,
    world::DeferredWorld,
};
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

use crate::components::Position;

/// A [`Resource`] containing all colliding entities.
#[derive(Debug, Default, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Debug, Default, Clone, PartialEq, Resource)]
pub struct EntityCollisions(EntityHashMap<EntityHashSet>);

impl EntityCollisions {
    /// Create a new, empty [`EntityCollisions`] resource.
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self(EntityHashMap::new()) }

    /// Push a pair of entities into the collision map.
    ///
    /// Returns `true` if the entities were not previously colliding.
    pub fn push_pair(&mut self, entity_a: Entity, entity_b: Entity) -> bool {
        let mut result = false;
        result |= self.0.entry(entity_a).or_default().insert(entity_b);
        result |= self.0.entry(entity_b).or_default().insert(entity_a);
        result
    }

    /// Remove a pair of entities from the collision map.
    ///
    /// Returns `true` if the entities were previously colliding.
    pub fn remove_pair(&mut self, entity_a: Entity, entity_b: Entity) -> bool {
        let mut result = false;
        result |= self.0.entry(entity_a).or_default().remove(&entity_b);
        result |= self.0.entry(entity_b).or_default().remove(&entity_a);
        result
    }

    /// Remove an entity from the collision map.
    ///
    /// Returns `true` if the entity was present in the collision map.
    pub fn remove_entity(&mut self, entity: Entity) -> bool {
        if let Some(collisions) = self.0.remove(&entity) {
            // Remove the entity from all of its collisions.
            for other in collisions {
                if let Some(other) = self.0.get_mut(&other) {
                    other.remove(&entity);
                }
            }
            true
        } else {
            // Search through all collisions in case the entity is present somewhere.
            let mut result = false;
            for collisions in self.0.values_mut() {
                result |= collisions.remove(&entity);
            }
            result
        }
    }

    /// Get the [`EntityHashSet`] of entities colliding with the given entity.
    #[must_use]
    pub fn get_collisions(&self, entity: Entity) -> Option<&EntityHashSet> { self.0.get(&entity) }

    /// Get the [`EntityHashSet`] of entities colliding with the given entity,
    /// or an empty set if the entity is not colliding with anything.
    #[must_use]
    pub fn get_collisions_or_empty(&mut self, entity: Entity) -> &EntityHashSet {
        self.0.entry(entity).or_default()
    }

    /// Returns `true` if the two entities are currently colliding.
    #[must_use]
    pub fn are_colliding(&self, entity_a: Entity, entity_b: Entity) -> bool {
        self.0.get(&entity_a).is_some_and(|set| set.contains(&entity_b))
            || self.0.get(&entity_b).is_some_and(|set| set.contains(&entity_a))
    }
}

// -------------------------------------------------------------------------------------------------

/// A [`Component`] containing colliding entities.
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(Debug, Default, Clone, PartialEq, Component)]
#[require(Position)]
#[component(on_discard = Self::discard_hook)]
pub struct CollidingWith(EntityHashSet);

impl CollidingWith {
    /// Create a new, empty [`CollidingWith`] component.
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self(EntityHashSet::new()) }

    /// A hook that removes the entity from the collision map when it is
    /// discarded.
    fn discard_hook(mut world: DeferredWorld, ctx: HookContext) {
        if let Some(mut collisions) = world.get_resource_mut::<EntityCollisions>() {
            collisions.remove_entity(ctx.entity);
        }
    }
}

impl AsRef<EntityHashSet> for CollidingWith {
    #[inline]
    fn as_ref(&self) -> &EntityHashSet { &self.0 }
}
impl AsMut<EntityHashSet> for CollidingWith {
    #[inline]
    fn as_mut(&mut self) -> &mut EntityHashSet { &mut self.0 }
}

impl Deref for CollidingWith {
    type Target = EntityHashSet;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for CollidingWith {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
