//! Player usernames and uuids.

use std::{borrow::Borrow, ops::Deref};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::Display;
use md5::{
    Md5,
    digest::{Digest, Update},
};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use uuid::{Builder, Uuid};

/// A player's username.
#[repr(transparent)]
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component), reflect(Component))]
pub struct PlayerUsername(SmolStr);

impl PlayerUsername {
    /// Create a new [`Username`] from a static string.
    #[must_use]
    pub const fn new_static(s: &'static str) -> Self { Self(SmolStr::new_static(s)) }

    /// Create a new [`Username`] from a string.
    #[must_use]
    pub fn new(s: impl Into<SmolStr>) -> Self { Self(s.into()) }

    /// Get the [`Username`] as a string slice.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str { self.0.as_str() }

    /// Get the offline [`Uuid`] of the player with this [`PlayerUsername`].
    ///
    /// # Example
    /// ```rust
    /// use froglight_player::prelude::PlayerUsername;
    ///
    /// let username = PlayerUsername::new_static("Mr_Sus_");
    /// assert_eq!(username.offline_uuid().to_string(), "fc6b8fd9-0dd1-399f-9924-3b08f51d4119");
    /// ```
    #[must_use]
    pub fn offline_uuid(&self) -> Uuid {
        Md5::new()
            .chain(format!("OfflinePlayer:{}", self.0))
            .finalize()
            .first_chunk::<16>()
            .map_or(Uuid::nil(), |&data| Builder::from_md5_bytes(data).into_uuid())
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
impl PlayerUsername {
    /// The Mojang username API endpoint.
    const MOJANG_USERNAME_API: &'static str =
        "https://api.minecraftservices.com/minecraft/profile/lookup/";

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
    /// use froglight_player::prelude::PlayerUsername;
    /// use uuid::Uuid;
    ///
    /// let agent = ureq::Agent::new_with_defaults();
    ///
    /// let uuid = Uuid::parse_str("352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3").unwrap();
    /// assert_eq!(PlayerUsername::username_of(&uuid, &agent).unwrap(), "Mr_Sus_");
    /// ```
    #[inline]
    pub fn username_of(uuid: &Uuid, agent: &ureq::Agent) -> Result<PlayerUsername, ureq::Error> {
        Self::username_at::<UsernameResponse>(uuid, Self::MOJANG_USERNAME_API, agent)
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
        Self::retry_request::<_, 3>(&format!("{api}{}", uuid.as_simple()), agent)
    }
}

// -------------------------------------------------------------------------------------------------

/// The response from the Mojang API when querying a player's UUID.
#[cfg(feature = "online")]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UuidResponse {
    /// The username of the player.
    #[serde(rename = "name")]
    pub username: String,
    /// The [`Uuid`] of the player.
    #[serde(rename = "id")]
    pub uuid: Uuid,
}

#[cfg(feature = "online")]
impl PlayerUsername {
    /// The Mojang UUID API endpoint.
    const MOJANG_UUID_API: &'static str =
        "https://api.minecraftservices.com/minecraft/profile/lookup/name/";

    /// Get the [`Uuid`] of the player with this [`Username`].
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
    /// use froglight_player::prelude::PlayerUsername;
    /// use uuid::Uuid;
    ///
    /// let agent = ureq::Agent::new_with_defaults();
    /// let username = PlayerUsername::new_static("Mr_Sus_");
    ///
    /// let uuid = username.player_uuid(&agent).unwrap();
    /// assert_eq!(uuid.to_string(), "352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3");
    /// ```
    pub fn player_uuid(&self, agent: &ureq::Agent) -> Result<Uuid, ureq::Error> {
        self.player_uuid_at::<UuidResponse>(Self::MOJANG_UUID_API, agent).map(|res| res.uuid)
    }

    /// Get the [`Uuid`] of the player with this [`Username`] and API.
    ///
    /// Will automatically retry up to 3 times if the request fails.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    ///
    /// # Errors
    /// Returns an error if the request fails.
    pub fn player_uuid_at<T: serde::de::DeserializeOwned>(
        &self,
        api: &(impl AsRef<str> + ?Sized),
        agent: &ureq::Agent,
    ) -> Result<T, ureq::Error> {
        Self::retry_request::<T, 3>(&format!("{}{}", api.as_ref(), self.0), agent)
    }

    /// Retry a request up to N times if it fails.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    fn retry_request<T: serde::de::DeserializeOwned, const N: usize>(
        uri: &str,
        agent: &ureq::Agent,
    ) -> Result<T, ureq::Error> {
        let mut response = Self::handle_request::<T>(uri, agent);

        // Retry up to N times if the request fails.
        let mut attempts = 0;
        while response.is_err() && attempts < N {
            response = Self::handle_request::<T>(uri, agent);
            attempts += 1;
        }

        response
    }

    /// Get player information from the given API endpoint.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    fn handle_request<T: serde::de::DeserializeOwned>(
        uri: &str,
        agent: &ureq::Agent,
    ) -> Result<T, ureq::Error> {
        agent.get(uri).call()?.into_body().read_json()
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
    #[cfg(feature = "online")]
    let agent = ureq::Agent::new_with_defaults();

    let username = PlayerUsername::new_static("Mr_Sus_");
    assert_eq!(username.offline_uuid().to_string(), "fc6b8fd9-0dd1-399f-9924-3b08f51d4119");

    #[cfg(feature = "online")]
    assert_eq!(
        username.player_uuid(&agent).unwrap().to_string(),
        "352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3"
    );
}
