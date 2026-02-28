//! TODO
#![allow(missing_docs, reason = "TODO")]
#![allow(clippy::struct_excessive_bools, reason = "Incorrect")]

use alloc::vec::Vec;

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
#[cfg(feature = "facet")]
use facet::Facet;
#[cfg(feature = "facet")]
use facet_minecraft as mc;
use froglight_common::{entity::EntityId, prelude::Identifier};
use froglight_player::prelude::{PlayerProfile, Username};
use uuid::Uuid;

use crate::common::spawn_info::PlayerSpawnInfo;

/// The content of a login hello packet.
///
/// Sent to the server to initiate the login process.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub struct LoginHelloContent {
    /// The player's username.
    pub username: Username,
    /// The player's UUID.
    pub uuid: Uuid,
}

impl LoginHelloContent {
    /// Create a new [`LoginHelloContent`].
    #[inline]
    #[must_use]
    pub const fn new(username: Username, uuid: Uuid) -> Self { Self { username, uuid } }

    /// Create a new [`LoginHelloContent`] from a [`PlayerProfile`].
    #[must_use]
    pub fn new_from_profile(profile: &PlayerProfile) -> Self {
        Self::new(profile.username().clone(), *profile.uuid())
    }

    /// Get the player's username.
    #[inline]
    #[must_use]
    pub const fn username(&self) -> &str { self.username.as_str() }

    /// Get the player's UUID.
    #[inline]
    #[must_use]
    pub const fn uuid(&self) -> Uuid { self.uuid }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct PlayLoginContent {
    pub player_id: EntityId,
    pub hardcore: bool,
    pub levels: Vec<Identifier<'static>>,
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub max_players: i32,
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub chunk_radius: u32,
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub simulation_distance: u32,
    pub reduced_info: bool,
    pub death_screen: bool,
    pub limited_crafting: bool,
    pub spawn_info: PlayerSpawnInfo,
    pub secure_chat: bool,
}
