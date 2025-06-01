#[cfg(not(feature = "std"))]
use alloc::sync::Arc;
use core::any::TypeId;
#[cfg(feature = "std")]
use std::sync::Arc;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::*;
use froglight_utils::storage::prelude::*;

use super::{EntityAttributeExt, EntityAttributeResolver, EntityAttributeTrait};

/// A dynamic storage for entity attributes.
///
/// Allows for the registration and retrieval of attributes at runtime.
#[repr(transparent)]
#[derive(Clone, AppStorage)]
#[storage(
    index(ident = "GlobalEntityAttributeId", inner = "u8"),
    bevy = "bevy",
    reflect = "reflect"
)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Clone))]
pub struct EntityAttributeStorage<V: Version>(
    IndexedLocalStorage<dyn EntityAttributeTrait<V>, GlobalEntityAttributeId>,
);

impl<V: Version> AppEntityAttributeStorage<V> {
    /// Create a new [`AppEntityAttributeStorage`] with the [`Vanilla`] types
    /// registered.
    #[must_use]
    pub fn new() -> Self
    where Vanilla: EntityAttributeResolver<V> {
        Self::from_storage(EntityAttributeStorage::new())
    }
}

impl<V: Version> EntityAttributeStorage<V> {
    /// Create a new [`EntityAttributeStorage`] with the [`Vanilla`] types
    /// registered.
    #[must_use]
    pub fn new() -> Self
    where Vanilla: EntityAttributeResolver<V> {
        let mut storage = Self::new_empty();
        <Vanilla as EntityAttributeResolver<V>>::register(&mut storage);
        storage
    }

    /// Create a new [`EntityAttributeStorage`] with no registered attributes.
    #[must_use]
    pub const fn new_empty() -> Self { Self(IndexedLocalStorage::new()) }

    /// Get the [`EntityAttributeTrait`] for the given
    /// [`GlobalEntityAttributeId`].
    ///
    /// Handy for storing many attribute types and bulk operations.
    ///
    /// Returns `None` if no attribute with the given id was registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_entity::{entity_attribute::GlobalEntityAttributeId, prelude::*};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new attribute storage.
    ///     let storage = EntityAttributeStorage::<V1_21_4>::new();
    ///
    ///     // Get the trait with the global id of `0`.
    ///     let attrib = storage.get_trait(GlobalEntityAttributeId::new_unchecked(0)).unwrap();
    ///     assert_eq!(attrib.identifier(), "minecraft:armor");
    ///
    ///     // Get the trait with the global id of `1`.
    ///     let attrib = storage.get_trait(GlobalEntityAttributeId::new_unchecked(1)).unwrap();
    ///     assert_eq!(attrib.identifier(), "minecraft:armor_toughness");
    /// }
    /// ```
    #[must_use]
    pub fn get_trait(
        &self,
        attrib_id: GlobalEntityAttributeId,
    ) -> Option<&'static dyn EntityAttributeTrait<V>> {
        self.0.get_index(attrib_id).map(|val| val.inner())
    }

    /// Get a entity attribute for the given attribute id.
    ///
    /// Returns `None` if no attribute with the given id was registered,
    /// or the attribute does not exist in the resolver.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_common::vanilla::Vanilla;
    /// use froglight_entity::{entity_attribute::GlobalEntityAttributeId, prelude::*};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///     use froglight_entity::entity_attribute::generated::v1_21_4::VersionEntityAttribute;
    ///
    ///     // Create a new attribute storage.
    ///     let storage = EntityAttributeStorage::<V1_21_4>::new();
    ///
    ///     // Get the attribute with the global id of `21`.
    ///     let attrib = storage.get_typed::<Vanilla>(GlobalEntityAttributeId::new_unchecked(21));
    ///     if let Some(VersionEntityAttribute::MovementSpeedAttribute(speed)) = &attrib {
    ///         assert_eq!(
    ///             EntityAttributeTrait::<V1_21_4>::identifier(speed),
    ///             "minecraft:movement_speed"
    ///         );
    ///     } else if attrib.is_some() {
    ///         panic!("EntityAttribute was not `MovementSpeed`, but {:?}!", attrib.unwrap());
    ///     }
    ///
    ///     // Get the attribute with the global id of `24`.
    ///     let attrib = storage.get_typed::<Vanilla>(GlobalEntityAttributeId::new_unchecked(24));
    ///     if let Some(VersionEntityAttribute::ScaleAttribute(scale)) = &attrib {
    ///         assert_eq!(EntityAttributeTrait::<V1_21_4>::identifier(scale), "minecraft:scale");
    ///     } else if attrib.is_some() {
    ///         panic!("EntityAttribute was not `Scale`, but {:?}!", attrib.unwrap());
    ///     }
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn get_typed<R: EntityAttributeResolver<V>>(
        &self,
        attrib_id: GlobalEntityAttributeId,
    ) -> Option<R::AttributeEnum> {
        self.get_trait(attrib_id).and_then(R::resolve)
    }

    /// Get the [`GlobalEntityAttributeId`] for the given entity attribute.
    ///
    /// Returns `None` if the attribute was not registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_entity::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new attribute storage.
    ///     let storage = EntityAttributeStorage::<V1_21_4>::new();
    ///
    ///     // Get the `GlobalEntityAttributeId` of `MaxHealthAttribute`.
    ///     let global_id = storage.get_global_id::<entity_attr::MaxHealthAttribute>().unwrap();
    ///     assert_eq!(*global_id, 18);
    ///
    ///     // Get the `GlobalEntityAttributeId` of `MaxAbsorptionAttribute`.
    ///     let global_id = storage.get_global_id::<entity_attr::MaxAbsorptionAttribute>().unwrap();
    ///     assert_eq!(*global_id, 17);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn get_global_id<E: EntityAttributeTrait<V>>(&self) -> Option<GlobalEntityAttributeId> {
        self.get_global_type_id(&TypeId::of::<E>())
    }

    /// Get the [`GlobalEntityAttributeId`] for the given entity attribute.
    ///
    /// Returns `None` if the attribute was not registered.
    #[must_use]
    pub fn get_global_type_id(&self, type_id: &TypeId) -> Option<GlobalEntityAttributeId> {
        self.0.get_index_of(type_id)
    }

    /// Register an entity attribute with the storage.
    ///
    /// This is required for converting between global ids and attribute types.
    ///
    /// # Note
    /// The order in which attributes are registered is important.
    ///
    /// If an attribute is registered out of order, all following attributes
    /// will have their global ids shifted incorrectly.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_entity::{entity_attribute::GlobalEntityAttributeId, prelude::*};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new attribute storage with the vanilla attributes registered.
    ///     let storage = EntityAttributeStorage::<V1_21_4>::new();
    ///
    ///     // Since `AttackDamage` is already registered, we can get its global id.
    ///     assert_eq!(
    ///         storage.get_global_id::<entity_attr::AttackDamageAttribute>(),
    ///         Some(GlobalEntityAttributeId::new_unchecked(2))
    ///     );
    ///
    ///     // Create a new empty attribute storage.
    ///     let mut storage = EntityAttributeStorage::<V1_21_4>::new_empty();
    ///
    ///     // Since `AttackDamage` is not registered, it does not have a global id.
    ///     assert_eq!(storage.get_global_id::<entity_attr::AttackDamageAttribute>(), None);
    ///
    ///     // Register the `AttackDamage` attribute, now we can get its global id.
    ///     storage.register::<entity_attr::AttackDamageAttribute>();
    ///     assert_eq!(
    ///         storage.get_global_id::<entity_attr::AttackDamageAttribute>(),
    ///         Some(GlobalEntityAttributeId::new_unchecked(0))
    ///     );
    /// }
    /// ```
    pub fn register<E: EntityAttributeExt<V>>(&mut self) {
        self.0.store(TypeId::of::<E>(), E::as_static());
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: Version> Default for AppEntityAttributeStorage<V>
where Vanilla: EntityAttributeResolver<V>
{
    fn default() -> Self { Self::new() }
}

impl<V: Version> Default for EntityAttributeStorage<V>
where Vanilla: EntityAttributeResolver<V>
{
    fn default() -> Self { Self::new() }
}

// -------------------------------------------------------------------------------------------------

impl From<usize> for GlobalEntityAttributeId {
    #[cfg(debug_assertions)]
    fn from(id: usize) -> Self {
        Self(u8::try_from(id).expect("GlobalEntityAttributeId is too large!"))
    }

    #[inline]
    #[cfg(not(debug_assertions))]
    #[expect(clippy::cast_possible_truncation)]
    fn from(id: usize) -> Self { Self(id as u8) }
}

impl From<GlobalEntityAttributeId> for usize {
    fn from(id: GlobalEntityAttributeId) -> Self { usize::from(id.0) }
}
