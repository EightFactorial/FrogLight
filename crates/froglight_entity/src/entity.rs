//! TODO

use core::{borrow::Borrow, fmt::Debug, ops::Deref};

use froglight_common::version::Version;

use crate::{info::EntityInfo, storage::Entities};

/// A trait implemented by all entity types, for each [`Version`] they exist.
pub trait EntityType<V: Version> {
    /// Get the [`EntityInfo`] for this entity type.
    fn info() -> &'static EntityInfo;
}

// -------------------------------------------------------------------------------------------------

/// An entity.
#[derive(Clone, Copy)]
pub struct Entity {
    entity_info: &'static EntityInfo,
}

impl Entity {
    /// Create a new [`Entity`] from an [`EntityType`] and [`Version`].
    #[inline]
    #[must_use]
    pub fn new<I: EntityType<V>, V: Version>() -> Self { Entity { entity_info: I::info() } }

    /// Create a new [`Entity`] from an [`EntityInfo`].
    #[inline]
    #[must_use]
    pub const fn new_from(entity_info: &'static EntityInfo) -> Self { Entity { entity_info } }

    /// Get the Entity's information.
    #[inline]
    #[must_use]
    pub const fn entity_info(&self) -> &'static EntityInfo { self.entity_info }
}

// impl Debug for Entity {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "Entity({}, {})", self.entity_info.identifier(),
// self.entity_info.base_id())     }
// }

// -------------------------------------------------------------------------------------------------

/// A entity's global id.
///
/// Contains the entity's id relative to all entities in the same [`Version`].
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalEntityId {
    entity_id: u32,
}

impl GlobalEntityId {
    /// Convert the [`GlobalEntityId`] into a [`Entity`].
    #[must_use]
    pub fn into_entity<V: Entities>(self) -> Option<Entity> {
        V::entities().read_blocking().get_entity(self)
    }

    /// Convert the [`GlobalEntityId`] into a [`EntityInfo`].
    #[must_use]
    pub fn into_info<V: Entities>(self) -> Option<&'static EntityInfo> {
        V::entities().read_blocking().get_info(self)
    }
}

impl AsRef<u32> for GlobalEntityId {
    fn as_ref(&self) -> &u32 { &self.entity_id }
}
impl Borrow<u32> for GlobalEntityId {
    fn borrow(&self) -> &u32 { &self.entity_id }
}

impl From<u32> for GlobalEntityId {
    fn from(entity_id: u32) -> Self { GlobalEntityId { entity_id } }
}
impl From<GlobalEntityId> for u32 {
    fn from(entity_id: GlobalEntityId) -> Self { entity_id.entity_id }
}

impl Debug for GlobalEntityId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GlobalEntityId({})", self.entity_id)
    }
}

impl Deref for GlobalEntityId {
    type Target = u32;

    fn deref(&self) -> &Self::Target { &self.entity_id }
}
