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
    pub textures: PlayerProfileTextures,
    /// The player's properties.
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    pub properties: HashMap<SmolStr, serde_json::Value>,
}

impl PlayerProfile {
    /// Create a new [`PlayerProfile`] from a username and [`Uuid`].
    #[must_use]
    pub fn new(username: impl Into<SmolStr>, uuid: Uuid) -> Self {
        Self::new_with_textures(PlayerUsername::new(username), uuid, PlayerProfileTextures::new())
    }

    /// Create a new [`PlayerProfile`] from a
    /// [`PlayerUsername`], [`Uuid`], and [`PlayerProfileTextures`].
    #[must_use]
    pub fn new_with_textures(
        username: PlayerUsername,
        uuid: Uuid,
        textures: PlayerProfileTextures,
    ) -> Self {
        Self { uuid, username, textures, properties: HashMap::new() }
    }

    /// Create a new offline [`PlayerProfile`] from a username.
    #[must_use]
    pub fn offline_profile(username: impl Into<SmolStr>) -> Self {
        let username = PlayerUsername::new(username);
        let uuid = username.offline_uuid();
        Self::new_with_textures(username, uuid, PlayerProfileTextures::new())
    }
}

// -------------------------------------------------------------------------------------------------

/// The textures of a player.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq))]
pub struct PlayerProfileTextures {
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

impl Default for PlayerProfileTextures {
    #[inline]
    fn default() -> Self { Self::new() }
}

impl PlayerProfileTextures {
    /// Create a new default [`PlayerProfileTextures`].
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

    fn try_from(response: ProfileResponse) -> Result<Self, Self::Error> {
        let mut properties = HashMap::new();
        let mut textures = None;

        for property in response.properties {
            if let Ok(data) = BASE64_URL_SAFE.decode(&property.value) {
                if let Ok(value) = serde_json::from_slice(&data) {
                    //

                    // If the property is "textures", parse it into a PlayerProfileTextures.
                    if property.name == "textures" {
                        match PlayerProfileTextures::try_from(&value) {
                            Ok(texts) => textures = Some(texts),
                            // Bevy logging.
                            #[cfg(feature = "bevy")]
                            Err(err) => bevy_log::warn!(
                                "Failed to parse \"{}\"'s textures, {err}",
                                response.username
                            ),
                            // No logging.
                            #[cfg(not(feature = "bevy"))]
                            Err(_) => {}
                        }
                    }

                    // Add the property to the properties map.
                    properties.insert(property.name, value);

                    // Skip warning below.
                    #[cfg(feature = "bevy")]
                    continue;
                }
            }

            #[cfg(feature = "bevy")]
            bevy_log::warn!(
                "Failed to decode \"{}\"'s profile at \"{}\"",
                response.username,
                property.value
            );
        }

        Ok(Self {
            properties,
            uuid: response.uuid,
            username: response.username.into(),
            textures: textures.unwrap_or_default(),
        })
    }
}

impl TryFrom<&serde_json::Value> for PlayerProfileTextures {
    type Error = serde::de::value::Error;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let Some(timestamp) = value.get("timestamp").and_then(serde_json::Value::as_u64) else {
            return Err(serde::de::Error::custom("\"timestamp\" not found"));
        };
        let Some(textures) = value.get("textures") else {
            return Err(serde::de::Error::custom("\"textures\" not found"));
        };

        Ok(Self {
            timestamp: SystemTime::UNIX_EPOCH
                .checked_add(Duration::from_millis(timestamp))
                .expect("SystemTime overflow!"),
            slim: textures
                .get("SKIN")
                .and_then(|skin| skin.get("model"))
                .and_then(serde_json::Value::as_str)
                .is_some_and(|model| model == "slim"),
            skin: textures
                .get("SKIN")
                .and_then(|skin| skin.get("url"))
                .and_then(serde_json::Value::as_str)
                .map(SmolStr::from),
            cape: textures
                .get("CAPE")
                .and_then(|cape| cape.get("url"))
                .and_then(serde_json::Value::as_str)
                .map(SmolStr::from),
        })
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[cfg(test)]
fn offline_profile() {
    let profile = PlayerProfile::offline_profile("Mr_Sus_");
    assert_eq!(profile.username, "Mr_Sus_");
    assert_eq!(profile.uuid.to_string(), "fc6b8fd9-0dd1-399f-9924-3b08f51d4119");
    assert_eq!(profile.textures.slim, false);
    assert!(profile.textures.skin.is_none());
    assert!(profile.textures.cape.is_none());
}

#[test]
#[cfg(test)]
#[cfg(feature = "online")]
fn online_profile() {
    let agent = ureq::Agent::new_with_defaults();
    let profile = PlayerProfile::profile_of_player("Mr_Sus_", &agent).unwrap();

    assert_eq!(profile.username, "Mr_Sus_");
    assert_eq!(profile.uuid.to_string(), "352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3");
    assert_eq!(profile.textures.slim, false);
    assert!(profile.textures.skin.is_some());
    assert!(profile.textures.cape.is_none());
}
