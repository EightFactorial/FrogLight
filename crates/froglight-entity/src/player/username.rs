//! [`PlayerUsername`]

#[cfg(not(feature = "std"))]
use alloc::{borrow::Borrow, format};
use core::ops::Deref;
#[cfg(feature = "std")]
use std::borrow::Borrow;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::Display;
use md5::{
    Md5,
    digest::{Digest, Update},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use uuid::{Builder, Uuid};

use super::uuid::PlayerUuid;

/// A player's username.
#[repr(transparent)]
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, PartialEq, Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct PlayerUsername(SmolStr);

impl PlayerUsername {
    /// Create a new [`PlayerUsername`] from a [`SmolStr`].
    #[must_use]
    pub const fn const_new(s: SmolStr) -> Self { Self(s) }

    /// Create a new [`PlayerUsername`] from a static string.
    #[must_use]
    pub const fn static_new(s: &'static str) -> Self { Self::const_new(SmolStr::new_static(s)) }

    /// Create a new [`PlayerUsername`] from a string.
    #[must_use]
    pub fn new(s: impl Into<SmolStr>) -> Self { Self(s.into()) }

    /// Get the offline [`Uuid`] of the player with this [`PlayerUsername`].
    ///
    /// # Example
    /// ```rust
    /// use froglight_entity::prelude::PlayerUsername;
    ///
    /// let username = PlayerUsername::static_new("Mr_Sus_");
    /// assert_eq!(username.offline_uuid().to_string(), "fc6b8fd9-0dd1-399f-9924-3b08f51d4119");
    /// ```
    #[must_use]
    pub fn offline_uuid(&self) -> PlayerUuid {
        PlayerUuid::new(
            Md5::new()
                .chain(format!("OfflinePlayer:{}", self.0))
                .finalize()
                .first_chunk::<16>()
                .map_or(Uuid::nil(), |&data| Builder::from_md5_bytes(data).into_uuid()),
        )
    }

    /// Get the [`Uuid`] of the player with this [`PlayerUsername`].
    ///
    /// Uses Mojang's API to fetch player data.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    ///
    /// # Errors
    /// Returns an error if the request to Mojang's API fails.
    ///
    /// # Example
    /// ```rust
    /// use froglight_entity::prelude::PlayerUsername;
    ///
    /// match PlayerUsername::static_new("Mr_Sus_").online_uuid(None) {
    ///     Ok(uuid) => assert_eq!(uuid.to_string(), "352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3"),
    ///     Err(err) => eprintln!("Failed to fetch UUID: {}", err),
    /// }
    /// ```
    #[cfg(feature = "online")]
    pub fn online_uuid(&self, agent: Option<&ureq::Agent>) -> Result<PlayerUuid, ureq::Error> {
        match agent {
            Some(agent) => self.player_online_uuid(agent),
            None => self.player_online_uuid(&ureq::Agent::new_with_defaults()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// The response from the Mojang API when querying a player's UUID.
#[cfg(feature = "online")]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UuidResponse {
    /// The username of the player.
    #[serde(rename = "name")]
    pub username: PlayerUsername,
    /// The [`Uuid`] of the player.
    #[serde(rename = "id")]
    pub uuid: PlayerUuid,
}

#[cfg(feature = "online")]
impl PlayerUsername {
    /// The Mojang UUID API endpoint.
    const MOJANG_UUID_API: &'static str =
        "https://api.minecraftservices.com/minecraft/profile/lookup/name/";

    /// Get the [`Uuid`] of the player with this [`PlayerUsername`].
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
    /// use froglight_entity::prelude::PlayerUsername;
    /// use uuid::Uuid;
    ///
    /// let agent = ureq::Agent::new_with_defaults();
    /// let username = PlayerUsername::static_new("Mr_Sus_");
    ///
    /// let uuid = username.player_online_uuid(&agent).unwrap();
    /// assert_eq!(uuid.to_string(), "352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3");
    /// ```
    pub fn player_online_uuid(&self, agent: &ureq::Agent) -> Result<PlayerUuid, ureq::Error> {
        Self::player_uuid_at::<UuidResponse>(self, Self::MOJANG_UUID_API, agent).map(|res| res.uuid)
    }

    /// Get the [`Uuid`] of the player with this [`PlayerUsername`] and API.
    ///
    /// Will automatically retry up to 3 times if the request fails.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    ///
    /// # Errors
    /// Returns an error if the request fails.
    pub fn player_uuid_at<T: serde::de::DeserializeOwned>(
        username: impl AsRef<str>,
        api: impl AsRef<str>,
        agent: &ureq::Agent,
    ) -> Result<T, ureq::Error> {
        super::retry_request::<T, 3>(&format!("{}{}", api.as_ref(), username.as_ref()), agent)
    }
}

// -------------------------------------------------------------------------------------------------

impl<T> From<T> for PlayerUsername
where SmolStr: From<T>
{
    fn from(s: T) -> Self { Self(SmolStr::from(s)) }
}

impl<T: ?Sized> PartialEq<T> for PlayerUsername
where SmolStr: PartialEq<T>
{
    fn eq(&self, other: &T) -> bool { self.0.eq(other) }
}

impl<T: ?Sized> AsRef<T> for PlayerUsername
where SmolStr: AsRef<T>
{
    fn as_ref(&self) -> &T { self.0.as_ref() }
}
impl<T: ?Sized> AsMut<T> for PlayerUsername
where SmolStr: AsMut<T>
{
    fn as_mut(&mut self) -> &mut T { self.0.as_mut() }
}

impl<T: ?Sized> Borrow<T> for PlayerUsername
where SmolStr: Borrow<T>
{
    fn borrow(&self) -> &T { self.0.borrow() }
}

impl<T: ?Sized> Deref for PlayerUsername
where SmolStr: Deref<Target = T>
{
    type Target = T;

    fn deref(&self) -> &T { &self.0 }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[cfg(test)]
fn username() {
    #[cfg(not(feature = "std"))]
    use alloc::string::ToString;

    let username = PlayerUsername::static_new("Mr_Sus_");
    assert_eq!(username.offline_uuid().to_string(), "fc6b8fd9-0dd1-399f-9924-3b08f51d4119");

    #[cfg(feature = "online")]
    {
        let uuid = username.online_uuid(None).unwrap();
        assert_eq!(uuid.to_string(), "352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3");
    }
}
