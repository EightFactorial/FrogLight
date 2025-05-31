#[cfg(not(feature = "std"))]
use alloc::sync::Arc;
use core::any::TypeId;
#[cfg(feature = "std")]
use std::sync::Arc;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use froglight_common::{vanilla::Vanilla, version::Version};
use froglight_utils::storage::prelude::*;

use super::{EntityTypeExt, EntityTypeResolver, EntityTypeTrait};

/// A dynamic storage for entity types.
///
/// Allows for the registration and retrieval of entity types at runtime.
#[repr(transparent)]
#[derive(Clone, AppStorage)]
#[storage(index(ident = "GlobalEntityTypeId", inner = "u16"), bevy = "bevy", reflect = "reflect")]
pub struct EntityTypeStorage<V: Version>(
    IndexedLocalStorage<dyn EntityTypeTrait<V>, GlobalEntityTypeId>,
);

impl<V: Version> AppEntityTypeStorage<V> {
    /// Create a new [`AppEntityTypeStorage`] with the [`Vanilla`] types
    /// registered.
    #[must_use]
    pub fn new() -> Self
    where Vanilla: EntityTypeResolver<V> {
        Self::from_storage(EntityTypeStorage::new())
    }
}

impl<V: Version> EntityTypeStorage<V> {
    /// Create a new [`EntityTypeStorage`] with the [`Vanilla`] types
    /// registered.
    #[must_use]
    pub fn new() -> Self
    where Vanilla: EntityTypeResolver<V> {
        let mut storage = Self::new_empty();
        <Vanilla as EntityTypeResolver<V>>::register(&mut storage);
        storage
    }

    /// Create a new [`EntityTypeStorage`] with no registered entity types.
    #[must_use]
    pub const fn new_empty() -> Self { Self(IndexedLocalStorage::new()) }

    /// Get the [`EntityTypeTrait`] for the given [`GlobalEntityTypeId`].
    ///
    /// Handy for storing many entity types and bulk operations.
    ///
    /// Returns `None` if no entity type with the given id was registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_entity::{entity_type::GlobalEntityTypeId, prelude::*};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new entity type storage.
    ///     let storage = EntityTypeStorage::<V1_21_4>::new();
    ///
    ///     // Get the trait with the global id of `0`.
    ///     let entity = storage.get_trait(GlobalEntityTypeId::new_unchecked(0)).unwrap();
    ///     assert_eq!(entity.identifier(), "minecraft:acacia_boat");
    ///
    ///     // Get the trait with the global id of `1`.
    ///     let entity = storage.get_trait(GlobalEntityTypeId::new_unchecked(1)).unwrap();
    ///     assert_eq!(entity.identifier(), "minecraft:acacia_chest_boat");
    /// }
    /// ```
    #[must_use]
    pub fn get_trait(
        &self,
        entity_type: GlobalEntityTypeId,
    ) -> Option<&'static dyn EntityTypeTrait<V>> {
        self.0.get_index(entity_type).map(|val| val.inner())
    }

    /// Get an entity type for the given entity type id.
    ///
    /// Returns `None` if no entity type with the given id was registered,
    /// or the entity type does not exist in the resolver.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_common::vanilla::Vanilla;
    /// use froglight_entity::{entity_type::GlobalEntityTypeId, prelude::*};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///     use froglight_entity::entity_type::generated::v1_21_4::VersionEntityType;
    ///
    ///     // Create a new entity type storage.
    ///     let storage = EntityTypeStorage::<V1_21_4>::new();
    ///
    ///     // Get the entity type with the global id of `0`.
    ///     let entity = storage.get_typed::<Vanilla>(GlobalEntityTypeId::new_unchecked(0));
    ///     if let Some(VersionEntityType::AcaciaBoat(boat)) = &entity {
    ///         assert_eq!(EntityTypeTrait::<V1_21_4>::identifier(boat), "minecraft:acacia_boat");
    ///     } else if entity.is_some() {
    ///         panic!("EntityType was not `AcaciaBoat`, but {:?}!", entity.unwrap());
    ///     }
    ///
    ///     // Get the entity type with the global id of `5`.
    ///     let entity = storage.get_typed::<Vanilla>(GlobalEntityTypeId::new_unchecked(5));
    ///     if let Some(VersionEntityType::ArmorStand(armorstand)) = &entity {
    ///         assert_eq!(EntityTypeTrait::<V1_21_4>::identifier(armorstand), "minecraft:armor_stand");
    ///     } else if entity.is_some() {
    ///         panic!("EntityType was not `ArmorStand`, but {:?}!", entity.unwrap());
    ///     }
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn get_typed<R: EntityTypeResolver<V>>(
        &self,
        entity_type: GlobalEntityTypeId,
    ) -> Option<R::EntityEnum> {
        self.get_trait(entity_type).and_then(R::resolve)
    }

    /// Get the [`GlobalEntityTypeId`] for the given entity type.
    ///
    /// Returns `None` if the entity type was not registered.
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
    ///     // Create a new entity type storage.
    ///     let storage = EntityTypeStorage::<V1_21_4>::new();
    ///
    ///     // Get the `GlobalEntityTypeId` of `Cat`.
    ///     let global_id = storage.get_global_id::<entity::Cat>().unwrap();
    ///     assert_eq!(*global_id, 20);
    ///
    ///     // Get the `GlobalEntityTypeId` of `Bat`.
    ///     let global_id = storage.get_global_id::<entity::Bat>().unwrap();
    ///     assert_eq!(*global_id, 10);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn get_global_id<E: EntityTypeTrait<V>>(&self) -> Option<GlobalEntityTypeId> {
        self.get_global_type_id(&TypeId::of::<E>())
    }

    /// Get the [`GlobalEntityTypeId`] for the given entity type.
    ///
    /// Returns `None` if the entity type was not registered.
    #[must_use]
    pub fn get_global_type_id(&self, type_id: &TypeId) -> Option<GlobalEntityTypeId> {
        self.0.get_index_of(type_id)
    }

    /// Register an entity type with the storage.
    ///
    /// This is required for converting between global ids and entity types.
    ///
    /// # Note
    /// The order in which entity types are registered is important.
    ///
    /// If an entity type is registered out of order, all following entity types
    /// will have their global ids shifted incorrectly.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_entity::{entity_type::GlobalEntityTypeId, prelude::*};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new entity type storage with the vanilla entity types registered.
    ///     let storage = EntityTypeStorage::<V1_21_4>::new();
    ///
    ///     // Since `Cat` is already registered, we can get its global id.
    ///     assert_eq!(
    ///         storage.get_global_id::<entity::Cat>(),
    ///         Some(GlobalEntityTypeId::new_unchecked(20))
    ///     );
    ///
    ///     // Create a new empty entity type storage.
    ///     let mut storage = EntityTypeStorage::<V1_21_4>::new_empty();
    ///
    ///     // Since `Cat` is not registered, it does not have a global id.
    ///     assert_eq!(storage.get_global_id::<entity::Cat>(), None);
    ///
    ///     // Register the `Cat` entity type, now we can get its global id.
    ///     storage.register::<entity::Cat>();
    ///     assert_eq!(
    ///         storage.get_global_id::<entity::Cat>(),
    ///         Some(GlobalEntityTypeId::new_unchecked(0))
    ///     );
    /// }
    /// ```
    pub fn register<E: EntityTypeExt<V>>(&mut self) {
        self.0.store(TypeId::of::<E>(), E::as_static());
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: Version> Default for AppEntityTypeStorage<V>
where Vanilla: EntityTypeResolver<V>
{
    fn default() -> Self { Self::new() }
}

impl<V: Version> Default for EntityTypeStorage<V>
where Vanilla: EntityTypeResolver<V>
{
    fn default() -> Self { Self::new() }
}

// -------------------------------------------------------------------------------------------------

impl From<usize> for GlobalEntityTypeId {
    #[cfg(debug_assertions)]
    fn from(id: usize) -> Self {
        Self(u16::try_from(id).expect("GlobalEntityTypeId is too large!"))
    }

    #[inline]
    #[cfg(not(debug_assertions))]
    #[expect(clippy::cast_possible_truncation)]
    fn from(id: usize) -> Self { Self(id as u16) }
}

impl From<GlobalEntityTypeId> for usize {
    fn from(id: GlobalEntityTypeId) -> Self { usize::from(id.0) }
}
