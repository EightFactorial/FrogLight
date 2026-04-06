use core::ops::{Deref, DerefMut};

use bevy_ecs::{
    entity::{EntityHashMap, EntityHashSet},
    prelude::*,
};
use bevy_reflect::prelude::*;
use bevy_transform::components::GlobalTransform;
use froglight_entity::prelude::EntityAabb;

use crate::prelude::*;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Debug, Default, Clone, PartialEq, Resource)]
pub struct EntityCollisions(EntityHashMap<EntityHashSet>);

impl Default for EntityCollisions {
    #[inline]
    fn default() -> Self { Self::new() }
}

impl EntityCollisions {
    /// Create a new, empty [`EntityCollisions`].
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
    /// Returns `true` if either entity were previously colliding.
    pub fn remove_pair(&mut self, entity_a: Entity, entity_b: Entity) -> bool {
        let mut result = false;
        result |= self.0.entry(entity_a).or_default().remove(&entity_b);
        result |= self.0.entry(entity_b).or_default().remove(&entity_a);
        result
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

    /// A [`System`] that updates the [`EntityCollisions`] [`Resource`] and the
    /// [`CollidingWith`] [`Components`](Component) of all [`Entities`](Entity).
    pub fn update_collisions(
        mut query: Query<(Entity, &EntityAabb, &GlobalTransform, &mut CollidingWith)>,
        mut collisions: ResMut<EntityCollisions>,
    ) {
        let mut iter = query.iter_combinations_mut();
        while let Some(
            [
                (entity_a, aabb_a, transform_a, mut colliding_a),
                (entity_b, aabb_b, transform_b, mut colliding_b),
            ],
        ) = iter.fetch_next()
        {
            let translated_a =
                aabb_a.with_translation(transform_a.translation()).with_scale(transform_a.scale());
            let translated_b =
                aabb_b.with_translation(transform_b.translation()).with_scale(transform_b.scale());

            if translated_a.intersects(&translated_b) {
                if collisions.push_pair(entity_a, entity_b) {
                    colliding_a.insert(entity_b);
                    colliding_b.insert(entity_a);
                }
            } else if collisions.remove_pair(entity_a, entity_b) {
                colliding_a.remove(&entity_b);
                colliding_b.remove(&entity_a);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect)]
#[require(Transform)]
#[reflect(Debug, Default, Clone, PartialEq, Component)]
pub struct CollidingWith(EntityHashSet);

impl Default for CollidingWith {
    #[inline]
    fn default() -> Self { Self::new() }
}

impl CollidingWith {
    /// Create a new, empty [`CollidingWith`].
    #[must_use]
    pub const fn new() -> Self { Self::from_set(EntityHashSet::new()) }

    /// Create a new [`CollidingWith`] from an existing [`EntityHashSet`].
    #[inline]
    #[must_use]
    pub const fn from_set(set: EntityHashSet) -> Self { Self(set) }
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
