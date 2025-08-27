//! TODO

use alloc::vec::Vec;
use core::{fmt::Debug, ops::Deref};

use froglight_common::version::Version;

use crate::{
    entity::{Entity, EntityType, GlobalEntityId},
    info::EntityInfo,
};

/// A [`Version`] with an associated [`EntityMap`].
pub trait Entities: Version {
    /// Get the [`StaticEntityMap`] for this [`Version`].
    fn entities() -> &'static StaticEntityMap;
    /// Initialize this version's entities into the provided [`EntityMap`].
    fn init_entities(map: &mut EntityMap);
}

// -------------------------------------------------------------------------------------------------

/// A modifiable, thread-safe reference to a [`EntityMap`].
#[repr(transparent)]
pub struct StaticEntityMap(
    #[cfg(feature = "async")] async_lock::RwLock<EntityMap>,
    #[cfg(not(feature = "async"))] parking_lot::RwLock<EntityMap>,
);

impl StaticEntityMap {
    /// Create a new [`StaticEntityMap`].
    #[must_use]
    #[cfg(feature = "async")]
    pub const fn new(map: EntityMap) -> Self { StaticEntityMap(async_lock::RwLock::new(map)) }

    /// Read the [`EntityMap`], blocking the current thread if necessary.
    #[must_use]
    #[cfg(feature = "async")]
    pub fn read_blocking(&self) -> async_lock::RwLockReadGuard<'_, EntityMap> {
        self.0.read_blocking()
    }

    /// Create a new [`StaticEntityMap`].
    #[must_use]
    #[cfg(not(feature = "async"))]
    pub const fn new(map: EntityMap) -> Self { StaticEntityMap(parking_lot::RwLock::new(map)) }

    /// Read the [`EntityMap`], blocking the current thread if necessary.
    #[cfg(not(feature = "async"))]
    pub fn read_blocking(&self) -> parking_lot::RwLockReadGuard<'_, EntityMap> { self.0.read() }
}

impl Debug for StaticEntityMap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticEntityMap").finish_non_exhaustive()
    }
}

impl Deref for StaticEntityMap {
    #[cfg(feature = "async")]
    type Target = async_lock::RwLock<EntityMap>;
    #[cfg(not(feature = "async"))]
    type Target = parking_lot::RwLock<EntityMap>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

// -------------------------------------------------------------------------------------------------

/// A list of static [`EntityInfo`]s.
///
/// Used for assigning ids to entities and retrieving their information.
pub struct EntityMap(Vec<&'static EntityInfo>);

impl EntityMap {
    /// Create a new empty [`EntityMap`].
    #[must_use]
    pub const fn new_empty() -> Self { EntityMap(Vec::new()) }

    /// Initialize the [`EntityMap`] with entities from the given version.
    #[inline]
    pub fn init<V: Entities>(&mut self) { V::init_entities(self); }

    /// Get a [`Entity`] for a given [`GlobalEntityId`].
    ///
    /// Returns `None` if the entity is not registered in the [`EntityMap`].
    #[must_use]
    pub fn get_entity(&self, entity: GlobalEntityId) -> Option<Entity> {
        self.get_info(entity).map(Entity::new_from)
    }

    /// Get the [`EntityInfo`] for a given [`GlobalEntityId`].
    ///
    /// Returns `None` if the entity is not registered in the [`EntityMap`].
    #[must_use]
    pub fn get_info(&self, entity: GlobalEntityId) -> Option<&'static EntityInfo> {
        self.0.get(*entity as usize).copied()
    }

    /// Get the number of entities registered in this [`EntityMap`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the [`EntityMap`] is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Register a [`EntityType`] in the [`EntityMap`].
    ///
    /// Assigns a [`GlobalEntityId`] to the [`EntityType`].
    #[inline]
    pub fn register<E: EntityType<V>, V: Version>(&mut self) { self.register_untyped(E::info()); }

    /// Register a [`EntityType`] in the [`EntityMap`].
    ///
    /// Assigns a [`GlobalEntityId`] to the [`EntityType`].
    // #[expect(
    //     clippy::cast_possible_truncation,
    //     reason = "There will never be 4,294,967,295 entity types"
    // )]
    pub fn register_untyped(&mut self, info: &'static EntityInfo) {
        // info.set_registered_id(self.0.len() as u32);
        self.0.push(info);
    }

    /// Get a reference to the inner [`Vec`] of the [`EntityMap`].
    ///
    /// Requires calling [`EntityMap::as_inner`] explicitly.
    #[inline]
    #[must_use]
    pub fn as_inner(map: &Self) -> &Vec<&'static EntityInfo> { &map.0 }

    /// Get a mutable reference to the inner [`Vec`] of the [`EntityMap`].
    ///
    /// Requires calling [`EntityMap::as_inner_mut`] explicitly.
    #[inline]
    #[must_use]
    pub fn as_inner_mut(map: &mut Self) -> &mut Vec<&'static EntityInfo> { &mut map.0 }
}
