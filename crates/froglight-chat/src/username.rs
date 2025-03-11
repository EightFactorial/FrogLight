//! Player [`Username`]s.

use std::ops::Deref;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use uuid::Uuid;

/// A player's username.
#[derive(
    Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into, Serialize, Deserialize,
)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component), reflect(Component))]
pub struct Username(SmolStr);

impl Username {
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

    /// Get the offline [`Uuid`] of the player with this [`Username`].
    pub fn offline_uuid(&self) -> Uuid { todo!() }

    /// Get the [`Uuid`] of the player with this [`Username`].
    ///
    /// Will automatically retry up to 3 times if the request fails.
    ///
    /// See the [Minecraft Wiki](https://minecraft.wiki/w/Mojang_API#Query_player_information)
    /// for more information.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    #[cfg(feature = "online")]
    pub fn player_uuid(&self, agent: &ureq::Agent) -> Result<Uuid, ureq::Error> {
        let uri = format!("https://api.mojang.com/users/profiles/minecraft/{}", self.0);
        let mut response = Self::player_request::<UuidResponse>(agent, &uri).map(|res| res.uuid);

        // Retry up to 3 times if the request fails.
        let mut attempts = 0;
        while response.is_err() && attempts < 3 {
            response = Self::player_request::<UuidResponse>(agent, &uri).map(|res| res.uuid);
            attempts += 1;
        }

        response
    }

    /// Get the player profile of the player with this [`Username`].
    ///
    /// Will automatically retry up to 3 times if the request fails.
    ///
    /// See the [Minecraft Wiki](https://minecraft.wiki/w/Mojang_API#Query_player_information)
    /// for more information.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    pub fn player_profile(&self, agent: &ureq::Agent) -> Result<UuidResponse, ureq::Error> {
        let uuid = self.player_uuid(agent)?;
        let uri = format!("https://sessionserver.mojang.com/session/minecraft/profile/{uuid}");
        let mut response = Self::player_request::<UuidResponse>(agent, &uri);

        // Retry up to 3 times if the request fails.
        let mut attempts = 0;
        while response.is_err() && attempts < 3 {
            response = Self::player_request::<UuidResponse>(agent, &uri);
            attempts += 1;
        }

        response
    }

    /// Get player information from the given API endpoint.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    #[cfg(feature = "online")]
    pub fn player_request<T: serde::de::DeserializeOwned>(
        agent: &ureq::Agent,
        uri: &str,
    ) -> Result<T, ureq::Error> {
        agent.get(uri).call()?.into_body().read_json()
    }
}

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

// -------------------------------------------------------------------------------------------------

impl<T: ?Sized> AsRef<T> for Username
where SmolStr: AsRef<T>
{
    fn as_ref(&self) -> &T { self.0.as_ref() }
}
impl<T: ?Sized> AsMut<T> for Username
where SmolStr: AsMut<T>
{
    fn as_mut(&mut self) -> &mut T { self.0.as_mut() }
}

impl<T: ?Sized> Deref for Username
where SmolStr: Deref<Target = T>
{
    type Target = T;

    fn deref(&self) -> &T { self.0.deref() }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[cfg(test)]
fn username() {
    #[cfg(feature = "online")]
    let agent = ureq::Agent::new_with_defaults();

    let username = Username::new_static("Mr_Sus_");
    // assert_eq!(username.offline_uuid().to_string(),
    // "352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3");

    #[cfg(feature = "online")]
    assert_eq!(
        username.player_uuid(&agent).unwrap().to_string(),
        "352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3"
    );
}
