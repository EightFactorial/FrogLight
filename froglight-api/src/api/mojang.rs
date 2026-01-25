use async_trait::async_trait;
use serde::Deserialize;
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
                    serde_json::from_slice::<NameAndUuid>(&response.data)?;

                Ok(Some(uuid))
            }
            Err(err) => Err(ApiError::Http(err)),
        }
    }

    async fn query_username(
        &self,
        uuid: Uuid,
        client: &HttpClient,
    ) -> Result<Option<String>, ApiError> {
        let url = format!("{}/{uuid}?unsigned=false", Self::USERNAME_ENDPOINT);
        match client.get(&url, GetOptions {}).await {
            // Note: Mojang returns a 404 for non-existent UUIDs.
            Ok(response) if response.status == 404 => Ok(None),
            Err(HttpError::Http(404)) => Ok(None),

            Ok(response) => {
                let NameAndUuid { name, .. } =
                    serde_json::from_slice::<NameAndUuid>(&response.data)?;

                Ok(Some(name))
            }
            Err(err) => Err(ApiError::Http(err)),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Deserialize)]
struct NameAndUuid {
    #[serde(rename = "id")]
    uuid: Uuid,
    name: String,
}
