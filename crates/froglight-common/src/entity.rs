//! [`EntityId`] and [`EntityUuid`] types.

use core::borrow::Borrow;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::derive::{AsRef, Deref, From, Into};
use uuid::Uuid;

/// A unique identifier for an [`Entity`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From, Into, AsRef, Deref)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash, Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct EntityId(u32);

impl Borrow<u32> for EntityId {
    fn borrow(&self) -> &u32 { &self.0 }
}

/// A universally unique identifier for an [`Entity`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From, Into, AsRef, Deref)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash, Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct EntityUuid(Uuid);

impl Borrow<Uuid> for EntityUuid {
    fn borrow(&self) -> &Uuid { &self.0 }
}
