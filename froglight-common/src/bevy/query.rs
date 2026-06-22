//! TODO

use core::{error, fmt, ops::Deref};

use bevy_ecs::{
    entity::Entity,
    query::{
        NestedQuery, QueryData, QueryEntityError, QueryFilter, QueryManyIter, ReadOnlyQueryData,
    },
};
use hashbrown::hash_map::Values;

use crate::prelude::{EntityId, EntityOfInstance, WorldInstance};

/// [`QueryData`] an entity's [`WorldInstance`].
///
/// # Example
///
/// ```rust
/// use bevy_ecs::prelude::*;
/// use froglight_common::prelude::*;
///
/// fn print_entities(query: Query<(Entity, Instance)>) {
///     for (current, instance) in query {
///         let identifier = instance.identifier();
///         println!("Entity {current} is in \"{identifier}\"!");
///     }
/// }
/// ```
#[derive(QueryData)]
pub struct Instance {
    entity_of: &'static EntityOfInstance,
    instances: NestedQuery<&'static WorldInstance>,
}

impl<'w> InstanceItem<'w, '_> {
    /// Get the [`WorldInstance`] of the current entity.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    #[must_use]
    pub fn instance(&self) -> &WorldInstance {
        match self.instances.get(self.entity_of.entity()) {
            Ok(instance) => instance,
            Err(err) => panic!("Failed to retrieve Entity's WorldInstance, {err}"),
        }
    }

    /// Get the [`WorldInstance`] of the current entity.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    #[must_use]
    pub fn instance_inner(self) -> &'w WorldInstance {
        match self.instances.get_inner(self.entity_of.entity()) {
            Ok(instance) => instance,
            Err(err) => panic!("Failed to retrieve Entity's WorldInstance, {err}"),
        }
    }

    /// Try to get the [`WorldInstance`] of the current entity.
    ///
    /// # Errors
    ///
    /// Returns an error if the current entity's [`WorldInstance`] cannot be
    /// retrieved.
    #[inline]
    pub fn try_instance(&self) -> Result<&WorldInstance, QueryEntityError> {
        self.instances.get(self.entity_of.entity())
    }
}

impl Deref for InstanceItem<'_, '_> {
    type Target = WorldInstance;

    #[inline]
    fn deref(&self) -> &Self::Target { self.instance() }
}

// -------------------------------------------------------------------------------------------------

/// [`QueryData`] for [`Components`] on the [`WorldInstance`] of the current
/// entity.
///
/// Note that this requires the inner query to be a read-only to prevent mutable
/// aliasing.
///
/// # Example
///
/// ```rust
/// use bevy_ecs::prelude::*;
/// use froglight_common::prelude::*;
///
/// fn print_entities(query: Query<(Entity, OnInstance<(Entity, &Name)>)>) {
///     for (current, instance) in query {
///         let (other, _name) = instance.get().unwrap();
///         println!("Entity {current} is in the Entity {other} instance!");
///     }
/// }
/// ```
#[derive(QueryData)]
pub struct OnInstance<D: ReadOnlyQueryData + 'static, F: QueryFilter + 'static = ()> {
    instance: Instance,
    query: NestedQuery<D, F>,
}

impl<'w, 's, D: ReadOnlyQueryData + 'static, F: QueryFilter + 'static>
    OnInstanceItem<'w, 's, D, F>
{
    /// Get the [`WorldInstance`] of the current entity.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    #[inline]
    #[must_use]
    pub fn instance(&self) -> &WorldInstance { self.instance.instance() }

    /// Try to get the [`WorldInstance`] of the current entity.
    ///
    /// # Errors
    ///
    /// Returns an error if the current entity's [`WorldInstance`] cannot be
    /// retrieved.
    #[inline]
    pub fn try_instance(&self) -> Result<&WorldInstance, QueryEntityError> {
        self.instance.try_instance()
    }

    /// Returns the read-only query item for the current entity.
    ///
    /// # Errors
    ///
    /// In case of a nonexisting entity or mismatched component, a
    /// [`QueryEntityError`] is returned instead.
    ///
    /// See [`Query::get`](bevy_ecs::system::Query::get) for more details.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    pub fn get(&self) -> Result<D::Item<'_, 's>, QueryEntityError> {
        self.query.get(self.instance.entity_of.entity())
    }

    /// Returns the query item for the current entity.
    /// This consumes the [`Query`] to return results with the actual "inner"
    /// world lifetime.
    ///
    /// # Errors
    ///
    /// In case of a nonexisting entity or mismatched component, a
    /// [`QueryEntityError`] is returned instead.
    ///
    /// See [`Query::get_inner`](bevy_ecs::system::Query::get_inner) for more
    /// details.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    pub fn get_inner(self) -> Result<D::Item<'w, 's>, QueryEntityError> {
        self.query.get_inner(self.instance.entity_of.entity())
    }
}

// -------------------------------------------------------------------------------------------------

/// [`QueryData`] for [`Components`] on entities in the same [`WorldInstance`]
/// as the current entity.
///
/// Note that this requires the inner query to be a read-only to prevent mutable
/// aliasing.
///
/// # Example
///
/// ```rust
/// use bevy_ecs::prelude::*;
/// use froglight_common::prelude::*;
///
/// fn print_entities(query: Query<(Entity, InInstance<(Entity, &Name)>)>) {
///     for (current, instance) in query {
///         for (other, _name) in instance {
///             println!("{current} is colliding with {other}!");
///         }
///     }
/// }
/// ```
#[derive(QueryData)]
pub struct InInstance<D: ReadOnlyQueryData + 'static, F: QueryFilter + 'static = ()> {
    instance: Instance,
    query: NestedQuery<D, F>,
}

impl<'w, 's, D: ReadOnlyQueryData, F: QueryFilter> InInstanceItem<'w, 's, D, F> {
    /// Get the [`WorldInstance`] of the current entity.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    #[inline]
    #[must_use]
    pub fn instance(&self) -> &WorldInstance { self.instance.instance() }

    /// Try to get the [`WorldInstance`] of the current entity.
    ///
    /// # Errors
    ///
    /// Returns an error if the current entity's [`WorldInstance`] cannot be
    /// retrieved.
    #[inline]
    pub fn try_instance(&self) -> Result<&WorldInstance, QueryEntityError> {
        self.instance.try_instance()
    }

    /// Returns the read-only query item for the given [`Entity`].
    ///
    /// # Errors
    ///
    /// If the entity is not part of the same [`WorldInstance`], a
    /// [`InInstanceError::Unknown`] is returned instead.
    ///
    /// In case of a nonexisting entity or mismatched component, a
    /// [`QueryEntityError`] is returned instead.
    ///
    /// See [`Query::get`](bevy_ecs::system::Query::get) for more details.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    pub fn get(&self, entity: Entity) -> Result<D::Item<'_, 's>, InInstanceError> {
        if self.instance().iter_entity().any(|e| *e == entity) {
            self.query.get(entity).map_err(InInstanceError::Query)
        } else {
            Err(InInstanceError::Unknown(entity))
        }
    }

    /// Returns the query item for the given [`Entity`].
    /// This consumes the [`Query`] to return results with the actual "inner"
    /// world lifetime.
    ///
    /// # Errors
    ///
    /// If the entity is not part of the same [`WorldInstance`], a
    /// [`InInstanceError::Unknown`] is returned instead.
    ///
    /// In case of a nonexisting entity or mismatched component, a
    /// [`QueryEntityError`] is returned instead.
    ///
    /// See [`Query::get_inner`](bevy_ecs::system::Query::get_inner) for more
    /// details.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    pub fn get_inner(self, entity: Entity) -> Result<D::Item<'w, 's>, InInstanceError> {
        if self.instance().iter_entity().any(|e| *e == entity) {
            self.query.get_inner(entity).map_err(InInstanceError::Query)
        } else {
            Err(InInstanceError::Unknown(entity))
        }
    }

    /// Returns the read-only query items for the given array of [`Entity`].
    ///
    /// The returned query items are in the same order as the input.
    ///
    /// # Errors
    ///
    /// If any of the entities are not part of the same [`WorldInstance`], a
    /// [`InInstanceError::Unknown`] is returned instead with the first
    /// non-colliding entity.
    ///
    /// In case of a nonexisting entity or mismatched component, a
    /// [`QueryEntityError`] is returned instead.
    ///
    /// See [`Query::get_many`](bevy_ecs::system::Query::get_many) for more
    /// details.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    pub fn get_many<const N: usize>(
        &self,
        entities: [Entity; N],
    ) -> Result<[D::Item<'_, 's>; N], InInstanceError> {
        let found = entities.map(|entity| self.instance().iter_entity().any(|e| *e == entity));

        for (entity, is_found) in entities.into_iter().zip(found) {
            if !is_found {
                return Err(InInstanceError::Unknown(entity));
            }
        }

        self.query.get_many(entities).map_err(InInstanceError::Query)
    }

    /// Returns the query items for the given array of [`Entity`].
    /// This consumes the [`Query`] to return results with the actual "inner"
    /// world lifetime.
    ///
    /// The returned query items are in the same order as the input.
    ///
    /// # Errors
    ///
    /// If any of the entities are not part of the same [`WorldInstance`], a
    /// [`InInstanceError::Unknown`] is returned instead with the first
    /// non-colliding entity.
    ///
    /// In case of a nonexisting entity or mismatched component, a
    /// [`QueryEntityError`] is returned instead.
    ///
    /// See [`Query::get_many_inner`](bevy_ecs::system::Query::get_many_inner)
    /// for more details.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    pub fn get_many_inner<const N: usize>(
        self,
        entities: [Entity; N],
    ) -> Result<[D::Item<'w, 's>; N], InInstanceError> {
        let found = entities.map(|entity| self.instance().iter_entity().any(|e| *e == entity));

        for (entity, is_found) in entities.into_iter().zip(found) {
            if !is_found {
                return Err(InInstanceError::Unknown(entity));
            }
        }

        self.query.get_many_inner(entities).map_err(InInstanceError::Query)
    }

    /// Returns an [`Iterator`] over the query items.
    ///
    /// This iterator is always guaranteed to return results from each matching
    /// entity once and only once. Iteration order is not guaranteed.
    ///
    /// See [`Query::iter_many`](bevy_ecs::system::Query::iter_many) for more
    /// details.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> QueryManyIter<'_, 's, D, F, Values<'_, EntityId, Entity>> {
        self.query.iter_many(self.instance().iter_entity())
    }

    /// Returns an [`Iterator`] over the query items.
    ///
    /// This iterator is always guaranteed to return results from each matching
    /// entity once and only once. Iteration order is not guaranteed.
    ///
    /// See [`Query::iter_many_inner`](bevy_ecs::system::Query::iter_many_inner)
    /// for more details.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`WorldInstance`] cannot be retrieved.
    #[inline]
    #[must_use]
    pub fn iter_inner(self) -> QueryManyIter<'w, 's, D, F, Values<'w, EntityId, Entity>> {
        let instance = self.instance.instance_inner();
        self.query.iter_many_inner(instance.iter_entity())
    }
}

// -------------------------------------------------------------------------------------------------

impl<'w, 's, D: ReadOnlyQueryData, F: QueryFilter> IntoIterator for InInstanceItem<'w, 's, D, F> {
    type IntoIter = QueryManyIter<'w, 's, D, F, Values<'w, EntityId, Entity>>;
    type Item = D::Item<'w, 's>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter_inner() }
}

impl<'iter, 's, D: ReadOnlyQueryData, F: QueryFilter> IntoIterator
    for &'iter InInstanceItem<'_, 's, D, F>
{
    type IntoIter = QueryManyIter<'iter, 's, D, F, Values<'iter, EntityId, Entity>>;
    type Item = D::Item<'iter, 's>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[allow(clippy::into_iter_without_iter, reason = "Read-Only")]
impl<'iter, 's, D: ReadOnlyQueryData, F: QueryFilter> IntoIterator
    for &'iter mut InInstanceItem<'_, 's, D, F>
{
    type IntoIter = QueryManyIter<'iter, 's, D, F, Values<'iter, EntityId, Entity>>;
    type Item = D::Item<'iter, 's>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

// -------------------------------------------------------------------------------------------------

/// An error returned by [`InInstance`] when a
/// [`Query`](bevy_ecs::system::Query) method fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InInstanceError {
    /// The requested entity is present in the same instance.
    Unknown(Entity),
    /// An error occurred while querying the entity.
    Query(QueryEntityError),
}

impl error::Error for InInstanceError {}
impl fmt::Display for InInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InInstanceError::Unknown(entity) => {
                write!(f, "Entity {entity} is not part of the same instance")
            }
            InInstanceError::Query(err) => {
                write!(f, "Failed to query entity, {err}")
            }
        }
    }
}
