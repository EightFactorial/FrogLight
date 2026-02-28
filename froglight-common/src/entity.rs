//! TODO
#![allow(clippy::unsafe_derive_deserialize, reason = "Triggered by `Facet` impl")]

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
#[cfg(feature = "facet")]
use facet::Facet;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An identifier for an entity.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", component(immutable))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
#[cfg_attr(feature = "facet", derive(Facet))]
#[cfg_attr(feature = "facet", facet(transparent))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct EntityId(pub i32);

impl EntityId {
    /// Create a new [`EntityId`].
    #[inline]
    #[must_use]
    pub const fn new(id: i32) -> Self { EntityId(id) }

    /// Get the inner [`i32`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> i32 { self.0 }
}

impl From<i32> for EntityId {
    fn from(value: i32) -> Self { EntityId(value) }
}
impl From<EntityId> for i32 {
    fn from(value: EntityId) -> Self { value.0 }
}

// -------------------------------------------------------------------------------------------------

/// A unique identifier for an entity.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", component(immutable))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
#[cfg_attr(feature = "facet", derive(Facet))]
#[cfg_attr(feature = "facet", facet(transparent))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct EntityUuid(pub Uuid);

impl EntityUuid {
    /// Create a new [`EntityUuid`].
    #[inline]
    #[must_use]
    pub const fn new(uuid: Uuid) -> Self { EntityUuid(uuid) }

    /// Get the inner [`Uuid`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> Uuid { self.0 }
}

impl From<Uuid> for EntityUuid {
    fn from(value: Uuid) -> Self { EntityUuid(value) }
}
impl From<EntityUuid> for Uuid {
    fn from(value: EntityUuid) -> Self { value.0 }
}
