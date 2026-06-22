use alloc::vec::Vec;
use core::{error, fmt};

use bevy_ecs::{
    entity::{Entity, hash_set::Iter as HashSetIter},
    query::{
        NestedQuery, QueryData, QueryEntityError, QueryFilter, QueryManyIter, ReadOnlyQueryData,
    },
    system::SystemParam,
};

use crate::bevy::CollidingWith;

/// [`QueryData`] for entities that are colliding with the current [`Entity`].
///
/// Used to query the [`Component`](bevy_ecs::component::Component)s of entities
/// that are colliding with the current entity.
///
/// Note that this requires the inner query to be a read-only to prevent mutable
/// aliasing.
///
/// # Example
///
/// ```rust
/// use bevy_ecs::prelude::*;
/// use froglight_physics::prelude::*;
///
/// fn print_collisions(query: Query<(Entity, Colliding<(Entity, &Position)>)>) {
///     for (current, colliding) in query {
///         for (other, _position) in colliding {
///             println!("{current} is colliding with {other}!");
///         }
///     }
/// }
/// ```
#[derive(QueryData)]
pub struct Colliding<D: ReadOnlyQueryData + 'static, F: QueryFilter + 'static = ()> {
    colliding: &'static CollidingWith,
    query: NestedQuery<D, F>,
}

impl<'w, 's, D: ReadOnlyQueryData, F: QueryFilter> CollidingItem<'w, 's, D, F> {
    /// Returns the read-only query item for the given [`Entity`].
    ///
    /// If the entity is not colliding with the current entity, a
    /// [`CollidingError::NotColliding`] is returned instead.
    ///
    /// In case of a nonexisting entity or mismatched component, a
    /// [`QueryEntityError`] is returned instead.
    ///
    /// This is always guaranteed to run in `O(1)` time.
    ///
    /// See [`Query::get`](bevy_ecs::system::Query::get) for more details.
    pub fn get(&self, entity: Entity) -> Result<D::Item<'_, 's>, CollidingError> {
        if self.colliding.contains(&entity) {
            self.query.get(entity).map_err(CollidingError::Query)
        } else {
            Err(CollidingError::NotColliding(entity))
        }
    }

    /// Returns the query item for the given [`Entity`].
    /// This consumes the [`Query`] to return results with the actual "inner"
    /// world lifetime.
    ///
    /// If the entity is not colliding with the current entity, a
    /// [`CollidingError::NotColliding`] is returned instead.
    ///
    /// In case of a nonexisting entity or mismatched component, a
    /// [`QueryEntityError`] is returned instead.
    ///
    /// This is always guaranteed to run in `O(1)` time.
    ///
    /// See [`Query::get_inner`](bevy_ecs::system::Query::get_inner) for more
    /// details.
    pub fn get_inner(self, entity: Entity) -> Result<D::Item<'w, 's>, CollidingError> {
        if self.colliding.contains(&entity) {
            self.query.get_inner(entity).map_err(CollidingError::Query)
        } else {
            Err(CollidingError::NotColliding(entity))
        }
    }

    /// Returns the read-only query items for the given array of [`Entity`].
    ///
    /// The returned query items are in the same order as the input.
    ///
    /// If any of the entities are not colliding with the current entity, a
    /// [`CollidingError::NotColliding`] is returned instead with the first
    /// non-colliding entity.
    ///
    /// In case of a nonexisting entity or mismatched component, a
    /// [`QueryEntityError`] is returned instead.
    ///
    /// See [`Query::get_many`](bevy_ecs::system::Query::get_many) for more
    /// details.
    pub fn get_many<const N: usize>(
        &self,
        entities: [Entity; N],
    ) -> Result<[D::Item<'_, 's>; N], CollidingError> {
        for entity in &entities {
            if !self.colliding.contains(entity) {
                return Err(CollidingError::NotColliding(*entity));
            }
        }

        self.query.get_many(entities).map_err(CollidingError::Query)
    }

    /// Returns the query items for the given array of [`Entity`].
    /// This consumes the [`Query`] to return results with the actual "inner"
    /// world lifetime.
    ///
    /// The returned query items are in the same order as the input.
    ///
    /// If any of the entities are not colliding with the current entity, a
    /// [`CollidingError::NotColliding`] is returned instead with the first
    /// non-colliding entity.
    ///
    /// In case of a nonexisting entity or mismatched component, a
    /// [`QueryEntityError`] is returned instead.
    ///
    /// See [`Query::get_many_inner`](bevy_ecs::system::Query::get_many_inner)
    /// for more details.
    pub fn get_many_inner<const N: usize>(
        self,
        entities: [Entity; N],
    ) -> Result<[D::Item<'w, 's>; N], CollidingError> {
        for entity in &entities {
            if !self.colliding.contains(entity) {
                return Err(CollidingError::NotColliding(*entity));
            }
        }

        self.query.get_many_inner(entities).map_err(CollidingError::Query)
    }

    /// Returns an [`Iterator`] over the query items.
    ///
    /// This iterator is always guaranteed to return results from each matching
    /// entity once and only once. Iteration order is not guaranteed.
    ///
    /// See [`Query::iter_many`](bevy_ecs::system::Query::iter_many) for more
    /// details.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> QueryManyIter<'_, 's, D, F, HashSetIter<'_>> {
        self.query.iter_many(self.colliding.iter())
    }

    /// Returns an [`Iterator`] over the query items, with the actual "inner"
    /// world lifetime.
    ///
    /// This iterator is always guaranteed to return results from each matching
    /// entity once and only once. Iteration order is not guaranteed.
    ///
    /// See [`Query::iter_many_inner`](bevy_ecs::system::Query::iter_many_inner)
    /// for more details.
    #[inline]
    #[must_use]
    pub fn iter_inner(self) -> QueryManyIter<'w, 's, D, F, HashSetIter<'w>> {
        self.query.iter_many_inner(self.colliding.iter())
    }
}

// -------------------------------------------------------------------------------------------------

impl<'w, 's, D: ReadOnlyQueryData, F: QueryFilter> IntoIterator for CollidingItem<'w, 's, D, F> {
    type IntoIter = QueryManyIter<'w, 's, D, F, HashSetIter<'w>>;
    type Item = D::Item<'w, 's>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter_inner() }
}

impl<'iter, 'w, 's, D: ReadOnlyQueryData, F: QueryFilter> IntoIterator
    for &'iter CollidingItem<'w, 's, D, F>
{
    type IntoIter = QueryManyIter<'iter, 's, D, F, HashSetIter<'iter>>;
    type Item = D::Item<'iter, 's>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl<'iter, 'w, 's, D: ReadOnlyQueryData, F: QueryFilter> IntoIterator
    for &'iter mut CollidingItem<'w, 's, D, F>
{
    type IntoIter = QueryManyIter<'iter, 's, D, F, HashSetIter<'iter>>;
    type Item = D::Item<'iter, 's>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

// -------------------------------------------------------------------------------------------------

/// An error returned by [`Colliding`] when a [`Query`](bevy_ecs::system::Query)
/// method fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollidingError {
    /// The requested entity is not colliding with the current entity.
    NotColliding(Entity),
    /// An error occurred while querying the entity.
    Query(QueryEntityError),
}

impl error::Error for CollidingError {}
impl fmt::Display for CollidingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollidingError::NotColliding(entity) => {
                write!(f, "Entity {entity} is not colliding with the current entity")
            }
            CollidingError::Query(err) => {
                write!(f, "Failed to query entity, {err}")
            }
        }
    }
}
