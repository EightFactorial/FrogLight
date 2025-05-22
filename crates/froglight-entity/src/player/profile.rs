//! [`PlayerProfile`] and [`PlayerProfileTextures`]

use core::time::Duration;

#[cfg(feature = "online")]
use base64::{Engine, prelude::BASE64_URL_SAFE};
#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "serde")]
use bevy_platform::hash::FixedHasher;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
#[cfg(feature = "serde")]
use indexmap::IndexMap;
#[cfg(feature = "online")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::{Serialize, de::DeserializeOwned};
use smol_str::SmolStr;
use uuid::Uuid;

use super::{username::PlayerUsername, uuid::PlayerUuid};

/// A player's profile.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, PartialEq, Component))]
pub struct PlayerProfile {
    /// The player's [`Uuid`].
    pub uuid: PlayerUuid,
    /// The player's username.
    pub username: PlayerUsername,
    /// The player's textures.
    pub textures: PlayerProfileTextures,

    /// The player's properties.
    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    pub properties: IndexMap<SmolStr, serde_json::Value, FixedHasher>,
}

impl PlayerProfile {
    /// Create a new [`PlayerProfile`] from a username and [`Uuid`].
    #[must_use]
    pub fn new(username: impl Into<SmolStr>, uuid: impl Into<Uuid>) -> Self {
        Self::new_with_textures(
            PlayerUsername::new(username),
            PlayerUuid::new(uuid),
            PlayerProfileTextures::new(),
        )
    }

    /// Create a new [`PlayerProfile`] from a
    /// [`PlayerUsername`], [`PlayerUuid`], and [`PlayerProfileTextures`].
    #[must_use]
    pub const fn new_with_textures(
        username: PlayerUsername,
        uuid: PlayerUuid,
        textures: PlayerProfileTextures,
    ) -> Self {
        #[cfg(feature = "serde")]
        {
            Self { uuid, username, textures, properties: IndexMap::with_hasher(FixedHasher) }
        }

        #[cfg(not(feature = "serde"))]
        {
            Self { uuid, username, textures }
        }
    }

    /// Insert a profile property into the [`PlayerProfile`].
    ///
    /// Optionally returns the previous value of the property.
    ///
    /// # Errors
    /// Returns an error if the property cannot be serialized.
    #[cfg(feature = "serde")]
    pub fn insert_property<T: Serialize>(
        &mut self,
        name: impl smol_str::ToSmolStr,
        property: &T,
    ) -> Result<Option<serde_json::Value>, serde_json::Error> {
        T::serialize(property, serde_json::value::Serializer)
            .map(|val| self.properties.insert(name.to_smolstr(), val))
    }

    /// Get a property from the [`PlayerProfile`].
    ///
    /// Returns `None` if the property does not exist.
    ///
    /// # Errors
    /// Returns an error if the property cannot be deserialized.
    #[must_use]
    #[cfg(feature = "serde")]
    pub fn get_property<T: DeserializeOwned>(
        &self,
        name: impl AsRef<str>,
    ) -> Option<Result<T, serde_json::Error>> {
        self.properties.get(name.as_ref()).map(|value| T::deserialize(value))
    }

    /// Take a property from the [`PlayerProfile`],
    /// shifting all following properties down by one.
    ///
    /// Returns `None` if the property does not exist.
    ///
    /// # Errors
    /// Returns an error if the property cannot be deserialized.
    #[cfg(feature = "serde")]
    pub fn shift_take_property<T: DeserializeOwned>(
        &mut self,
        name: impl AsRef<str>,
    ) -> Option<Result<T, serde_json::Error>> {
        self.properties.shift_remove(name.as_ref()).map(|value| T::deserialize(value))
    }

    /// Take a property from the [`PlayerProfile`],
    /// swapping the property with the last one and removing it.
    ///
    /// Returns `None` if the property does not exist.
    ///
    /// # Errors
    /// Returns an error if the property cannot be deserialized.
    #[cfg(feature = "serde")]
    pub fn swap_take_property<T: DeserializeOwned>(
        &mut self,
        name: impl AsRef<str>,
    ) -> Option<Result<T, serde_json::Error>> {
        self.properties.swap_remove(name.as_ref()).map(|value| T::deserialize(value))
    }

    /// Create a new offline [`PlayerProfile`] from a username.
    #[must_use]
    pub fn offline_profile(username: impl Into<SmolStr>) -> Self {
        let username = PlayerUsername::new(username);
        let uuid = username.offline_uuid();
        Self::new_with_textures(username, uuid, PlayerProfileTextures::new())
    }

    /// Create a new [`PlayerProfile`] from a username.
    ///
    /// Uses Mojang's API to fetch player data.
    ///
    /// # Warning
    /// This function will block until the request is complete!
    ///
    /// # Errors
    /// Returns an error if the request to Mojang's API fails.
    #[cfg(feature = "online")]
    pub fn online_profile(
        username: impl Into<SmolStr>,
        agent: Option<&ureq::Agent>,
    ) -> Result<Self, ureq::Error> {
        match agent {
            Some(agent) => Self::profile_of_player(username, agent),
            None => Self::profile_of_player(username, &ureq::Agent::new_with_defaults()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// The textures of a player.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, PartialEq))]
pub struct PlayerProfileTextures {
    /// The timestamp the textures were retrieved.
    pub timestamp: Duration,
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
    /// Create a new empty [`PlayerProfileTextures`].
    #[must_use]
    pub const fn new() -> Self {
        Self { timestamp: Duration::ZERO, slim: false, skin: None, cape: None }
    }
}

// -------------------------------------------------------------------------------------------------

/// The response from the Mojang API when querying a player's profile.
#[cfg(feature = "online")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct ProfileResponse {
    /// The username of the player.
    #[serde(rename = "name")]
    pub username: PlayerUsername,
    /// The [`Uuid`] of the player.
    #[serde(rename = "id")]
    pub uuid: PlayerUuid,
    /// The properties of the player.
    pub properties: Vec<ProfileResponseProperty>,
}

/// A property of a player's profile.
#[cfg(feature = "online")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
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
    /// use froglight_entity::prelude::PlayerProfile;
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
        Self::profile_of(&PlayerUsername::new(username).player_online_uuid(agent)?, agent)
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
    /// use froglight_entity::prelude::{PlayerProfile, PlayerUuid};
    /// use uuid::Uuid;
    ///
    /// let agent = ureq::Agent::new_with_defaults();
    /// let uuid: PlayerUuid = Uuid::parse_str("352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3").unwrap().into();
    ///
    /// let profile = PlayerProfile::profile_of(&uuid, &agent).unwrap();
    /// assert_eq!(profile.username, "Mr_Sus_");
    /// assert_eq!(profile.uuid, uuid);
    /// ```
    #[expect(clippy::missing_panics_doc)]
    pub fn profile_of(
        uuid: &PlayerUuid,
        agent: &ureq::Agent,
    ) -> Result<PlayerProfile, ureq::Error> {
        Self::profile_at::<ProfileResponse>(uuid.as_ref(), Self::MOJANG_PROFILE_API, agent)
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
        super::retry_request::<T, 3>(&format!("{api}{}", uuid.as_simple()), agent)
    }
}

#[cfg(feature = "online")]
impl TryFrom<ProfileResponse> for PlayerProfile {
    type Error = serde::de::value::Error;

    fn try_from(response: ProfileResponse) -> Result<Self, Self::Error> {
        let mut properties = IndexMap::with_capacity_and_hasher(
            response.properties.len().saturating_sub(1),
            FixedHasher,
        );

        let mut textures = None;
        for property in response.properties {
            if let Ok(data) = BASE64_URL_SAFE.decode(&property.value)
                && let Ok(value) = serde_json::from_slice(&data)
            {
                //

                // If the property is "textures", parse it into a PlayerProfileTextures.
                if property.name == "textures"
                    && let Ok(tex) = PlayerProfileTextures::try_from(&value)
                {
                    textures = Some(tex);
                }

                // Add the property to the properties map.
                properties.insert(property.name, value);
            }
        }

        Ok(Self {
            properties,
            uuid: response.uuid,
            username: response.username,
            textures: textures.unwrap_or_default(),
        })
    }
}

#[cfg(feature = "serde")]
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
            timestamp: Duration::from_millis(timestamp),
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
    #[cfg(not(feature = "std"))]
    use alloc::string::ToString;

    let profile = PlayerProfile::offline_profile("Mr_Sus_");
    assert_eq!(profile.username, "Mr_Sus_");
    assert_eq!(profile.uuid.to_string(), "fc6b8fd9-0dd1-399f-9924-3b08f51d4119");
    assert!(!profile.textures.slim);
    assert!(profile.textures.skin.is_none());
    assert!(profile.textures.cape.is_none());
}

#[test]
#[cfg(test)]
#[cfg(feature = "online")]
fn online_profile() {
    let profile = PlayerProfile::online_profile("Mr_Sus_", None).unwrap();

    assert_eq!(profile.username, "Mr_Sus_");
    assert_eq!(profile.uuid.to_string(), "352f97ab-cb6a-4bdf-aedc-c8764b8f6fc3");
    assert!(!profile.textures.slim);
    assert!(profile.textures.skin.is_some());
    assert!(profile.textures.cape.is_none());
}
