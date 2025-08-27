//! [`EntityId`] and [`EntityUuid`] types.

use core::borrow::Borrow;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A unique identifier for an [`Entity`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct EntityId(pub u32);

impl AsRef<u32> for EntityId {
    fn as_ref(&self) -> &u32 { &self.0 }
}
impl Borrow<u32> for EntityId {
    fn borrow(&self) -> &u32 { &self.0 }
}

impl From<u32> for EntityId {
    fn from(id: u32) -> Self { Self(id) }
}
impl From<EntityId> for u32 {
    fn from(id: EntityId) -> Self { id.0 }
}

// -------------------------------------------------------------------------------------------------

/// A universally unique identifier for an [`Entity`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct EntityUuid(pub Uuid);

impl AsRef<Uuid> for EntityUuid {
    fn as_ref(&self) -> &Uuid { &self.0 }
}
impl Borrow<Uuid> for EntityUuid {
    fn borrow(&self) -> &Uuid { &self.0 }
}

impl From<Uuid> for EntityUuid {
    fn from(uuid: Uuid) -> Self { Self(uuid) }
}
impl From<EntityUuid> for Uuid {
    fn from(uuid: EntityUuid) -> Self { uuid.0 }
}
