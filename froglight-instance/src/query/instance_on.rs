use bevy_ecs::query::{
    NestedQuery, QueryData, QueryEntityError, QueryFilter, ReadOnlyQueryData, With,
};

use crate::{instance::SessionInstance, relationship::PartOfInstance};

/// [`QueryData`] for [`Components`] on the [`SessionInstance`] of the current
/// entity.
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
/// fn print_entities(query: Query<(Entity, OnInstance<(Entity, Option<&Name>)>)>) {
///     for (current, instance) in query {
///         let Ok((other, _name)) = instance.get() else { continue };
///         println!("Entity {current} is in Entity {other}'s instance!");
///     }
/// }
/// ```
#[derive(QueryData)]
pub struct OnInstance<D: ReadOnlyQueryData + 'static, F: QueryFilter + 'static = ()> {
    of_instance: &'static PartOfInstance,
    query: NestedQuery<D, (With<SessionInstance>, F)>,
}

impl<'w, 's, D: ReadOnlyQueryData + 'static, F: QueryFilter + 'static>
    OnInstanceItem<'w, 's, D, F>
{
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
    /// Panics if the current entity's [`SessionInstance`] cannot be retrieved.
    #[inline]
    pub fn get(&self) -> Result<D::Item<'_, 's>, QueryEntityError> {
        self.query.get(self.of_instance.instance())
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
    /// Panics if the current entity's [`SessionInstance`] cannot be retrieved.
    #[inline]
    pub fn get_inner(self) -> Result<D::Item<'w, 's>, QueryEntityError> {
        self.query.get_inner(self.of_instance.instance())
    }
}
