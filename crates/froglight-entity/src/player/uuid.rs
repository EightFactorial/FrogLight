//! [`PlayerUUID`]

#[cfg(not(feature = "std"))]
use alloc::borrow::Borrow;
use core::ops::{Deref, DerefMut};
#[cfg(feature = "std")]
use std::borrow::Borrow;

#[cfg(feature = "bevy")]
use bevy_ecs::{component::HookContext, prelude::*, world::DeferredWorld};
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::Display;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "online")]
use smol_str::SmolStr;
use uuid::Uuid;

#[cfg(feature = "online")]
use super::username::PlayerUsername;

/// A player's [`Uuid`].
#[repr(transparent)]
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(feature = "bevy", derive(Component), component(on_add = Self::on_add), reflect(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct PlayerUuid(Uuid);

impl PlayerUuid {
    /// Create a new [`PlayerUuid`] from a [`Uuid`].
    #[must_use]
    pub const fn const_new(uuid: Uuid) -> Self { Self(uuid) }

    /// Create a new [`PlayerUuid`] from a [`Uuid`].
    #[must_use]
    pub fn new(uuid: impl Into<Uuid>) -> Self { Self(uuid.into()) }

    /// Insert the [`PlayerUuid`] as an [`EntityUuid`].
    ///
    /// Only occurs the first time a [`PlayerUuid`] is added to an [`Entity`],
    /// any changes to the [`PlayerUuid`] will not affect the [`EntityUuid`].
    #[cfg(feature = "bevy")]
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        if let Some(uuid) = world.get::<Self>(ctx.entity) {
            let uuid = froglight_common::entity::EntityUuid::from(uuid.0);
            world.commands().entity(ctx.entity).insert(uuid);
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// The response from the Mojang API when querying a player's username.
#[cfg(feature = "online")]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UsernameResponse {
    /// The username of the player.
    #[serde(rename = "name")]
    pub username: SmolStr,
    /// The [`Uuid`] of the player.
    #[serde(rename = "id")]
    pub uuid: Uuid,
}

#[cfg(feature = "online")]
impl PlayerUuid {
    /// The Mojang username API endpoint.
    const MOJANG_USERNAME_API: &'static str =
        "https://api.minecraftservices.com/minecraft/profile/lookup/";

    /// Get the [`PlayerUsername`] of the player with the given [`Uuid`].
    ///
    /// Uses Mojang's API to fetch player data.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    ///
    /// # Errors
    /// Returns an error if the request to Mojang's API fails.
    #[cfg(feature = "online")]
    pub fn online_username(
        &self,
        agent: Option<&ureq::Agent>,
    ) -> Result<PlayerUsername, ureq::Error> {
        match agent {
            Some(agent) => self.username_of(agent),
            None => self.username_of(&ureq::Agent::new_with_defaults()),
        }
    }

    /// Get the username of the player with the given [`Uuid`].
    ///
    /// Will automatically retry up to 3 times if the request fails.
    ///
    /// See the [Minecraft Wiki](https://minecraft.wiki/w/Mojang_API#Query_player_information)
    /// for more information.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    ///
    /// # Errors
    /// Returns an error if the request fails.
    ///
    /// # Example
    /// ```rust
    /// use froglight_entity::prelude::{PlayerUsername, PlayerUuid};
    /// use uuid::Uuid;
    ///
    /// let agent = ureq::Agent::new_with_defaults();
    ///
    /// let uuid: PlayerUuid = Uuid::parse_str("352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3").unwrap().into();
    /// assert_eq!(uuid.username_of(&agent).unwrap(), "Mr_Sus_");
    /// ```
    #[inline]
    pub fn username_of(&self, agent: &ureq::Agent) -> Result<PlayerUsername, ureq::Error> {
        Self::username_at::<UsernameResponse>(self.as_ref(), Self::MOJANG_USERNAME_API, agent)
            .map(|res| PlayerUsername::from(res.username))
    }

    /// Get the username of the player with the given [`Uuid`] and API.
    ///
    /// Will automatically retry up to 3 times if the request fails.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    ///
    /// # Errors
    /// Returns an error if the request fails.
    pub fn username_at<T: serde::de::DeserializeOwned>(
        uuid: &Uuid,
        api: &str,
        agent: &ureq::Agent,
    ) -> Result<T, ureq::Error> {
        super::retry_request::<_, 3>(&format!("{api}{}", uuid.as_simple()), agent)
    }
}

// -------------------------------------------------------------------------------------------------

impl<T> From<T> for PlayerUuid
where Uuid: From<T>
{
    fn from(s: T) -> Self { Self(Uuid::from(s)) }
}

impl<T: ?Sized> PartialEq<T> for PlayerUuid
where Uuid: PartialEq<T>
{
    fn eq(&self, other: &T) -> bool { self.0.eq(other) }
}

impl<T: ?Sized> AsRef<T> for PlayerUuid
where Uuid: AsRef<T>
{
    fn as_ref(&self) -> &T { self.0.as_ref() }
}
impl<T: ?Sized> AsMut<T> for PlayerUuid
where Uuid: AsMut<T>
{
    fn as_mut(&mut self) -> &mut T { self.0.as_mut() }
}

impl<T: ?Sized> Borrow<T> for PlayerUuid
where Uuid: Borrow<T>
{
    fn borrow(&self) -> &T { self.0.borrow() }
}

impl Deref for PlayerUuid {
    type Target = Uuid;

    fn deref(&self) -> &Uuid { &self.0 }
}
impl DerefMut for PlayerUuid {
    fn deref_mut(&mut self) -> &mut Uuid { &mut self.0 }
}
