use core::any::TypeId;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
use bevy_platform::{
    collections::{
        HashMap,
        hash_map::{IntoValues, Values},
    },
    hash::NoOpHash,
};
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use froglight_common::version::Version;

use super::{EntityAttributeTrait, generated::EntityAttribute};

/// A set of [`EntityAttribute`]s.
///
/// Allows for storage of multiple different types of attributes.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub struct EntityAttributeSet(HashMap<TypeId, EntityAttribute, NoOpHash>);

impl EntityAttributeSet {
    /// Create a new empty [`EntityAttributeSet`].
    #[must_use]
    pub const fn new() -> Self { Self(HashMap::with_hasher(NoOpHash)) }

    /// Insert an [`EntityAttribute`] into the set.
    ///
    /// Returns the previous value if it was already present.
    pub fn insert<E: EntityAttributeTrait<V> + Into<EntityAttribute>, V: Version>(
        &mut self,
        attribute: E,
    ) -> Option<EntityAttribute> {
        self.0.insert(TypeId::of::<E>(), attribute.into())
    }

    /// Remove an [`EntityAttribute`] from the [`EntityAttributeSet`].
    #[inline]
    #[must_use]
    pub fn remove<E: EntityAttributeTrait<V>, V: Version>(&mut self) -> Option<EntityAttribute> {
        self.remove_type(TypeId::of::<E>())
    }

    /// Remove an [`EntityAttribute`] from the [`EntityAttributeSet`]
    /// by its [`TypeId`].
    #[must_use]
    pub fn remove_type(&mut self, type_id: TypeId) -> Option<EntityAttribute> {
        self.0.remove(&type_id)
    }

    /// Returns `true` if the set contains no attributes.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Insert all attributes in the [`EntityAttributeSet`]
    /// into an [`Entity`](bevy_ecs::entity::Entity).
    #[cfg(feature = "bevy")]
    pub fn apply_to(self, entity: &mut EntityWorldMut) {
        self.into_iter().for_each(|attr| attr.apply_to(entity));
    }

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is `&'a V`.
    ///
    /// Refer to [`values`](hb::HashMap::values) for further details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bevy_platform::collections::HashMap;
    /// #
    /// let mut map = HashMap::new();
    ///
    /// map.insert("foo", 0);
    /// map.insert("bar", 1);
    /// map.insert("baz", 2);
    ///
    /// for key in map.values() {
    ///     // 0, 1, 2
    ///     // Note that the above order is not guaranteed
    /// }
    /// #
    /// # assert_eq!(map.values().count(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Values<'_, TypeId, EntityAttribute> { self.values() }

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is `&'a V`.
    ///
    /// Refer to [`values`](hb::HashMap::values) for further details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bevy_platform::collections::HashMap;
    /// #
    /// let mut map = HashMap::new();
    ///
    /// map.insert("foo", 0);
    /// map.insert("bar", 1);
    /// map.insert("baz", 2);
    ///
    /// for key in map.values() {
    ///     // 0, 1, 2
    ///     // Note that the above order is not guaranteed
    /// }
    /// #
    /// # assert_eq!(map.values().count(), 3);
    /// ```
    #[must_use]
    pub fn values(&self) -> Values<'_, TypeId, EntityAttribute> { self.0.values() }

    /// Creates a consuming iterator visiting all the values in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `V`.
    ///
    /// Refer to [`into_values`](hb::HashMap::into_values) for further details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bevy_platform::collections::HashMap;
    /// #
    /// let mut map = HashMap::new();
    ///
    /// map.insert("foo", 0);
    /// map.insert("bar", 1);
    /// map.insert("baz", 2);
    ///
    /// for key in map.into_values() {
    ///     // 0, 1, 2
    ///     // Note that the above order is not guaranteed
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn into_values(self) -> IntoValues<TypeId, EntityAttribute> { self.0.into_values() }
}

impl IntoIterator for EntityAttributeSet {
    type IntoIter = IntoValues<TypeId, EntityAttribute>;
    type Item = EntityAttribute;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.into_values() }
}

impl<'a> IntoIterator for &'a EntityAttributeSet {
    type IntoIter = Values<'a, TypeId, EntityAttribute>;
    type Item = &'a EntityAttribute;

    fn into_iter(self) -> Self::IntoIter { self.values() }
}
