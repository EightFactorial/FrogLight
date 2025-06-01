use core::any::TypeId;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
use bevy_platform::{collections::HashMap, hash::NoOpHash};
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
        self.0.into_iter().for_each(|(_, attr)| attr.apply_to(entity));
    }
}
