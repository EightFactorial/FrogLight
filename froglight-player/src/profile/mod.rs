//! A player's [`PlayerProfile`]
#![allow(clippy::unsafe_derive_deserialize, reason = "Triggered by deriving `facet` and `serde`")]

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::username::Username;

mod partial;
pub use partial::*;

mod property;
pub use property::*;

/// A player's profile
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", component(on_add = Self::add_hook))]
#[cfg_attr(feature = "bevy", reflect(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct PlayerProfile {
    /// The player's [`Uuid`].
    ///
    /// This is not the same as the player's
    /// [`EntityUuid`](froglight_common::entity::EntityUuid).
    uuid: Uuid,
    /// The player's username.
    ///
    /// The real name of the player, should not be changed.
    username: Username,
    /// The player's profile properties.
    properties: ProfilePropertySet,
}

impl PlayerProfile {
    /// Creates a new [`PlayerProfile`] with the given [`Uuid`] and
    /// [`Username`].
    #[must_use]
    pub fn new(uuid: Uuid, username: Username) -> Self {
        Self { uuid, username, properties: ProfilePropertySet::new() }
    }

    /// Creates a new [`PlayerProfile`] for an offline player with the given
    /// [`Username`].
    #[must_use]
    pub fn new_offline(username: Username) -> Self {
        let uuid = username.uuid_offline();
        Self::new(uuid, username)
    }

    /// Get a reference to the player's [`Username`].
    #[inline]
    #[must_use]
    pub const fn username(&self) -> &Username { &self.username }

    /// Get a reference to the player's [`Uuid`].
    #[inline]
    #[must_use]
    pub const fn uuid(&self) -> &Uuid { &self.uuid }

    /// Get a reference to the [`ProfilePropertySet`].
    #[inline]
    #[must_use]
    pub const fn properties(&self) -> &ProfilePropertySet { &self.properties }

    /// Get a mutable reference to the [`ProfilePropertySet`].
    #[inline]
    #[must_use]
    pub const fn properties_mut(&mut self) -> &mut ProfilePropertySet { &mut self.properties }

    /// An `on_add` hook that inserts a [`Username`] component
    /// if the entity does not already have one.
    #[cfg(feature = "bevy")]
    fn add_hook(mut world: bevy_ecs::world::DeferredWorld, ctx: bevy_ecs::lifecycle::HookContext) {
        if let Ok(entity) = world.get_entity(ctx.entity)
            && !entity.contains::<Username>()
            && let Some(profile) = entity.get::<Self>()
        {
            let username = profile.username().clone();
            world.commands().entity(ctx.entity).insert(username);
        }
    }
}
