//! Player profiles and textures.

use std::time::{Duration, SystemTime};

use base64::{Engine, prelude::BASE64_URL_SAFE};
#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use hashbrown::HashMap;
use serde::Deserialize;
use smol_str::SmolStr;
use uuid::Uuid;

use crate::username::PlayerUsername;

/// A player's profile.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect), reflect(Debug, PartialEq, Component))]
pub struct PlayerProfile {
    /// The player's [`Uuid`].
    pub uuid: Uuid,
    /// The player's username.
    pub username: PlayerUsername,
    /// The player's textures.
    pub textures: PlayerTextures,
    /// The player's properties.
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    pub properties: HashMap<SmolStr, serde_json::Value>,
}

impl PlayerProfile {
    /// Create a new [`PlayerProfile`] from a username and [`Uuid`].
    #[must_use]
    pub fn new(username: impl Into<SmolStr>, uuid: Uuid) -> Self {
        Self::new_with_textures(PlayerUsername::new(username), uuid, PlayerTextures::new())
    }

    /// Create a new offline [`PlayerProfile`] from a username.
    #[must_use]
    pub fn new_offline(username: impl Into<SmolStr>) -> Self {
        let username = PlayerUsername::new(username);
        let uuid = username.offline_uuid();
        Self::new_with_textures(username, uuid, PlayerTextures::new())
    }

    /// Create a new [`PlayerProfile`] from a
    /// [`PlayerUsername`], [`Uuid`], and [`PlayerTextures`].
    #[must_use]
    pub fn new_with_textures(
        username: PlayerUsername,
        uuid: Uuid,
        textures: PlayerTextures,
    ) -> Self {
        Self { uuid, username, textures, properties: HashMap::new() }
    }
}

// -------------------------------------------------------------------------------------------------

/// The textures of a player.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq))]
pub struct PlayerTextures {
    /// The timestamp the textures were retrieved.
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    pub timestamp: SystemTime,
    /// Whether the player has a slim model.
    pub slim: bool,
    /// The URL of the player's skin.
    pub skin: Option<SmolStr>,
    /// The URL of the player's cape.
    pub cape: Option<SmolStr>,
}

impl Default for PlayerTextures {
    #[inline]
    fn default() -> Self { Self::new() }
}

impl PlayerTextures {
    /// Create a new default [`PlayerTextures`].
    #[must_use]
    pub fn new() -> Self {
        Self { timestamp: SystemTime::now(), slim: false, skin: None, cape: None }
    }
}

// -------------------------------------------------------------------------------------------------

/// The response from the Mojang API when querying a player's profile.
#[cfg(feature = "online")]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ProfileResponse {
    /// The username of the player.
    #[serde(rename = "name")]
    pub username: SmolStr,
    /// The [`Uuid`] of the player.
    #[serde(rename = "id")]
    pub uuid: Uuid,
    /// The properties of the player.
    pub properties: Vec<ProfileResponseProperty>,
}

/// A property of a player's profile.
#[cfg(feature = "online")]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ProfileResponseProperty {
    /// The name of the property.
    pub name: SmolStr,
    /// The value of the property.
    pub value: String,
}

#[cfg(feature = "online")]
impl PlayerProfile {
    /// The Mojang profile API endpoint.
    const MOJANG_PROFILE_API: &'static str =
        "https://sessionserver.mojang.com/session/minecraft/profile/";

    /// Get the player profile of the player with the given [`PlayerUsername`].
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
    /// use froglight_player::prelude::PlayerProfile;
    /// use uuid::Uuid;
    ///
    /// let agent = ureq::Agent::new_with_defaults();
    ///
    /// let profile = PlayerProfile::profile_of_player("Mr_Sus_", &agent).unwrap();
    /// assert_eq!(profile.username, "Mr_Sus_");
    /// assert_eq!(profile.uuid.to_string(), "352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3");
    /// ```
    pub fn profile_of_player(
        username: impl Into<SmolStr>,
        agent: &ureq::Agent,
    ) -> Result<PlayerProfile, ureq::Error> {
        Self::profile_of(&PlayerUsername::new(username).player_uuid(agent)?, agent)
    }

    /// Get the player profile of the player with the given [`Uuid`].
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
    /// use froglight_player::prelude::PlayerProfile;
    /// use uuid::Uuid;
    ///
    /// let agent = ureq::Agent::new_with_defaults();
    /// let uuid = Uuid::parse_str("352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3").unwrap();
    ///
    /// let profile = PlayerProfile::profile_of(&uuid, &agent).unwrap();
    /// assert_eq!(profile.username, "Mr_Sus_");
    /// assert_eq!(profile.uuid, uuid);
    /// ```
    #[expect(clippy::missing_panics_doc)]
    pub fn profile_of(uuid: &Uuid, agent: &ureq::Agent) -> Result<PlayerProfile, ureq::Error> {
        Self::profile_at::<ProfileResponse>(uuid, Self::MOJANG_PROFILE_API, agent)
            .map(|res| res.try_into().expect("Mojang API returned invalid response!"))
    }

    /// Get the player profile of the player with the given [`Uuid`] and API.
    ///
    /// Will automatically retry up to 3 times if the request fails.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    ///
    /// # Errors
    /// Returns an error if the request fails.
    pub fn profile_at<T: serde::de::DeserializeOwned>(
        uuid: &Uuid,
        api: &str,
        agent: &ureq::Agent,
    ) -> Result<T, ureq::Error> {
        Self::retry_request::<T, 3>(&format!("{api}{}", uuid.as_simple()), agent)
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

impl TryFrom<ProfileResponse> for PlayerProfile {
    type Error = serde::de::value::Error;

    fn try_from(value: ProfileResponse) -> Result<Self, Self::Error> {
        let mut textures = PlayerTextures::new();
        let mut properties = HashMap::new();

        for property in value.properties {
            if let Ok(data) = BASE64_URL_SAFE.decode(&property.value) {
                if let Ok(value) = serde_json::from_slice(&data) {
                    // Check if the property is the textures property.
                    if property.name == "textures" {
                        textures = PlayerTextures::try_from(&value)?;
                    }
                    // Add the property to the properties map.
                    properties.insert(property.name, value);
                }
            }
        }

        Ok(Self { textures, properties, uuid: value.uuid, username: value.username.into() })
    }
}

impl TryFrom<&serde_json::Value> for PlayerTextures {
    type Error = serde::de::value::Error;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(Self {
            timestamp: {
                let duration = Duration::from_millis(value["timestamp"].as_u64().unwrap());
                SystemTime::UNIX_EPOCH.checked_add(duration).expect("SystemTime overflow!")
            },
            slim: {
                if let Some(skin) = value["textures"].get("SKIN") {
                    skin.get("metadata")
                        .and_then(|meta| meta.get("model"))
                        .is_some_and(|model| model.as_str() == Some("slim"))
                } else {
                    false
                }
            },
            skin: {
                if let Some(skin) = value["textures"].get("SKIN") {
                    skin.get("url").map(|url| url.as_str().unwrap().into())
                } else {
                    None
                }
            },
            cape: {
                if let Some(cape) = value["textures"].get("CAPE") {
                    cape.get("url").map(|url| url.as_str().unwrap().into())
                } else {
                    None
                }
            },
        })
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[cfg(test)]
#[cfg(feature = "online")]
fn profile() {
    let agent = ureq::Agent::new_with_defaults();

    let username = PlayerUsername::new_static("Mr_Sus_");
    let uuid = username.player_uuid(&agent).unwrap();

    let profile = PlayerProfile::profile_of(&uuid, &agent).unwrap();

    assert_eq!(username, profile.username);
    assert_eq!(uuid, profile.uuid);
    assert_eq!(profile.textures.slim, false);
    assert!(profile.textures.skin.is_some());
    assert!(profile.textures.cape.is_none());
}
