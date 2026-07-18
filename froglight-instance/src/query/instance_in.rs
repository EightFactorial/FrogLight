use alloc::boxed::Box;
use core::fmt;

use bevy_ecs::{
    entity::Entity,
    query::{
        NestedQuery, QueryData, QueryEntityError, QueryFilter, ReadOnlyQueryData,
        SingleEntityQueryData, Without,
    },
    resource::IsResource,
};

use crate::{instance::SessionInstance, relationship::PartOfInstance};

/// [`QueryData`] for [`Entities`](Entity) with [`Components`] in the
/// [`SessionInstance`] of the current entity.
///
/// Note that this requires the inner query to be a read-only to prevent mutable
/// aliasing.
///
/// # Example
///
/// ```rust
/// use bevy_ecs::prelude::*;
/// use froglight_instance::prelude::*;
///
/// fn print_entities(query: Query<(Entity, InInstance<(Entity, Option<&Name>)>)>) {
///     for (current, instance) in query {
///         for (other, name) in instance.iter() {
///             println!("Entity {current} is in the same instance as Entity {other}!");
///         }
///     }
/// }
/// ```
#[derive(QueryData)]
pub struct InInstance<
    D: ReadOnlyQueryData + SingleEntityQueryData + 'static,
    F: QueryFilter + 'static = (),
> {
    of_instance: &'static PartOfInstance,
    instances: NestedQuery<&'static SessionInstance, Without<IsResource>>,
    query: NestedQuery<(&'static PartOfInstance, D), F>,
}

impl<'w, 's, D: ReadOnlyQueryData + SingleEntityQueryData + 'static, F: QueryFilter + 'static>
    InInstanceItem<'w, 's, D, F>
{
    /// Get the [`SessionInstance`] of the current entity.
    ///
    /// # Errors
    ///
    /// Returns a [`QueryEntityError`] if the [`PartOfInstance`] points to an
    /// [`Entity`] without a [`SessionInstance`].
    #[inline]
    pub fn try_instance(&self) -> Result<&SessionInstance, QueryEntityError> {
        self.instances.get(self.of_instance.instance())
    }

    /// Get the [`SessionInstance`] of the current entity.
    ///
    /// # Panics
    ///
    /// Panics if the [`PartOfInstance`] points to an [`Entity`] without a
    /// [`SessionInstance`].
    #[inline]
    pub fn instance(&self) -> &SessionInstance {
        self.try_instance().expect("Could not get Entity's SessionInstance!")
    }

    /// Returns the read-only query item for the current entity.
    ///
    /// # Errors
    ///
    /// If the provided [`Entity`] does not have the required components, a
    /// [`QueryEntityError`] is returned. See
    /// [`Query::get`](bevy_ecs::system::Query::get) for more details.
    ///
    /// If the provided [`Entity`] does not belong to the same
    /// [`SessionInstance`] as the current entity, an
    /// [`InInstanceError::InstanceMismatch`] is returned.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`SessionInstance`] cannot be retrieved.
    pub fn get(&self, entity: Entity) -> Result<D::Item<'_, 's>, InInstanceError> {
        let (entity, query) = self.query.get(entity)?;
        if self.of_instance.instance() == entity.instance() {
            Ok(query)
        } else {
            Err(InInstanceError::InstanceMismatch(*self.of_instance, *entity))
        }
    }

    /// Returns the read-only query item for the current entity.
    ///
    /// # Errors
    ///
    /// If the provided [`Entity`] does not have the required components, a
    /// [`QueryEntityError`] is returned. See
    /// [`Query::get`](bevy_ecs::system::Query::get) for more details.
    ///
    /// If the provided [`Entity`] does not belong to the same
    /// [`SessionInstance`] as the current entity, an
    /// [`InInstanceError::InstanceMismatch`] is returned.
    ///
    /// # Panics
    ///
    /// Panics if the current entity's [`SessionInstance`] cannot be retrieved.
    pub fn get_inner(self, entity: Entity) -> Result<D::Item<'w, 's>, InInstanceError> {
        let (entity, query) = self.query.get_inner(entity)?;
        if self.of_instance.instance() == entity.instance() {
            Ok(query)
        } else {
            Err(InInstanceError::InstanceMismatch(*self.of_instance, *entity))
        }
    }

    /// Returns an iterator over all matching entities in the same
    /// [`SessionInstance`].
    #[must_use]
    pub fn iter(&self) -> InInstanceIter<'_, 's, D> {
        InInstanceIter {
            iter: Box::new(self.query.iter().filter_map(|(entity, query)| {
                if self.of_instance.instance() == entity.instance() { Some(query) } else { None }
            })),
        }
    }

    /// Returns an iterator over all matching entities in the same
    /// [`SessionInstance`].
    #[must_use]
    pub fn iter_inner<'ws>(self) -> InInstanceIter<'ws, 's, D>
    where
        'w: 'ws,
        's: 'ws,
    {
        InInstanceIter {
            iter: Box::new(self.query.into_iter().filter_map(|(entity, query)| {
                if self.of_instance.instance() == entity.instance() { Some(query) } else { None }
            })),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// An iterator over an [`InInstance`] query.
#[repr(transparent)]
pub struct InInstanceIter<'w, 's, D: ReadOnlyQueryData + SingleEntityQueryData + 'static> {
    iter: Box<dyn Iterator<Item = D::Item<'w, 's>> + 'w>,
}

impl<'w, 's, D: ReadOnlyQueryData + SingleEntityQueryData + 'static> Iterator
    for InInstanceIter<'w, 's, D>
{
    type Item = D::Item<'w, 's>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { self.iter.next() }
}

impl<'iter, 's, D: ReadOnlyQueryData + SingleEntityQueryData + 'static> IntoIterator
    for &'iter InInstanceItem<'_, 's, D, ()>
{
    type IntoIter = InInstanceIter<'iter, 's, D>;
    type Item = D::Item<'iter, 's>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl<'iter, 's, D: ReadOnlyQueryData + SingleEntityQueryData + 'static> IntoIterator
    for &'iter mut InInstanceItem<'_, 's, D, ()>
{
    type IntoIter = InInstanceIter<'iter, 's, D>;
    type Item = D::Item<'iter, 's>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'w, 's, D: ReadOnlyQueryData + SingleEntityQueryData + 'static, F: QueryFilter + 'static>
    IntoIterator for InInstanceItem<'w, 's, D, F>
where
    'w: 's,
    's: 'w,
{
    type IntoIter = InInstanceIter<'w, 's, D>;
    type Item = D::Item<'w, 's>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter_inner() }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InInstanceError {
    InstanceMismatch(PartOfInstance, PartOfInstance),
    Query(QueryEntityError),
}

impl core::error::Error for InInstanceError {}
impl fmt::Display for InInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InInstanceError::InstanceMismatch(current, other) => {
                write!(
                    f,
                    "Entity {} is not part of the same SessionInstance as Entity {}!",
                    current.instance(),
                    other.instance()
                )
            }
            InInstanceError::Query(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl From<QueryEntityError> for InInstanceError {
    #[inline]
    fn from(value: QueryEntityError) -> Self { InInstanceError::Query(value) }
}
