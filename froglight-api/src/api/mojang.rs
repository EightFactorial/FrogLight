use async_trait::async_trait;
use facet::Facet;
use froglight_player::{
    prelude::{PlayerProfile, Username},
    profile::ProfileProperty,
};
use uuid::Uuid;

use crate::{
    api::{ApiError, NetworkApi},
    client::{GetOptions, HttpClient, HttpError},
};

/// The standard, default [`NetworkApi`].
///
/// Uses Mojang's official API endpoints.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mojang;

impl Mojang {
    /// The Mojang API endpoint for querying player profiles.
    pub const PROFILE_ENDPOINT: &'static str =
        "https://sessionserver.mojang.com/session/minecraft/profile";
    /// The Mojang API endpoint for querying usernames by UUID.
    pub const USERNAME_ENDPOINT: &'static str =
        "https://sessionserver.mojang.com/session/minecraft/profile";
    /// The Mojang API endpoint for querying UUIDs by username.
    pub const UUID_ENDPOINT: &'static str = "https://api.mojang.com/users/profiles/minecraft";
}

#[async_trait]
impl NetworkApi for Mojang {
    async fn query_uuid(
        &self,
        username: &str,
        client: &HttpClient,
    ) -> Result<Option<Uuid>, ApiError> {
        let url = format!("{}/{username}", Self::UUID_ENDPOINT);
        match client.get(&url, GetOptions {}).await {
            // Note: Mojang returns a 404 for non-existent usernames.
            Ok(response) if response.status == 404 => Ok(None),
            Err(HttpError::Http(404)) => Ok(None),

            Ok(response) => {
                let NameAndUuid { uuid, .. } =
                    facet_json::from_slice::<NameAndUuid>(&response.data)?;

                Ok(Some(uuid))
            }
            Err(err) => Err(ApiError::Http(err)),
        }
    }

    async fn query_username(
        &self,
        uuid: Uuid,
        client: &HttpClient,
    ) -> Result<Option<Username>, ApiError> {
        let url = format!("{}/{uuid}?unsigned=false", Self::USERNAME_ENDPOINT);
        match client.get(&url, GetOptions {}).await {
            // Note: Mojang returns a 404 for non-existent UUIDs.
            Ok(response) if response.status == 404 => Ok(None),
            Err(HttpError::Http(404)) => Ok(None),

            Ok(response) => {
                let NameAndUuid { name, .. } =
                    facet_json::from_slice::<NameAndUuid>(&response.data)?;

                Ok(Some(Username::new(name)))
            }
            Err(err) => Err(ApiError::Http(err)),
        }
    }

    async fn query_profile(
        &self,
        uuid: Uuid,
        client: &HttpClient,
    ) -> Result<Option<PlayerProfile>, ApiError> {
        let url = format!("{}/{uuid}?unsigned=false", Self::PROFILE_ENDPOINT);
        match client.get(&url, GetOptions {}).await {
            // Note: Mojang returns a 204 for non-existent profiles.
            Ok(response) if response.status == 204 => Ok(None),
            Err(HttpError::Http(204)) => Ok(None),

            Ok(response) => {
                let api_profile = facet_json::from_slice::<ApiProfile>(&response.data)?;

                let mut profile = PlayerProfile::new(api_profile.uuid, api_profile.name.into());
                for property in api_profile.properties {
                    profile.properties_mut().insert(
                        property.name,
                        ProfileProperty { value: property.value, signature: property.signature },
                    );
                }

                Ok(Some(profile))
            }
            Err(err) => Err(ApiError::Http(err)),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Facet)]
struct NameAndUuid {
    #[facet(rename = "id")]
    uuid: Uuid,
    name: String,
}

#[derive(Facet)]
struct ApiProfile {
    #[facet(rename = "id")]
    uuid: Uuid,
    name: String,
    properties: Vec<ApiProfileProperty>,
}
#[derive(Facet)]
struct ApiProfileProperty {
    name: String,
    value: String,
    signature: Option<String>,
}
