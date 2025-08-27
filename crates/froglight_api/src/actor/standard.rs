//! TODO

use alloc::{boxed::Box, format, string::String, vec::Vec};
use core::net::IpAddr;

use base64::{Engine, prelude::BASE64_STANDARD};
use foldhash::fast::RandomState;
use indexmap::IndexMap;
use rsa::{RsaPublicKey, pkcs8::DecodePublicKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    actor::{
        ActorError, ClientActor, PlayerProfile,
        types::{ApiPublicKeys, ServerId},
    },
    backend::ClientBackend,
};

/// The standard [`ApiClient`](crate::client::ApiClient) actor.
///
/// Uses Mojang's API and [`serde`] for deserialization.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Standard;

#[async_trait::async_trait]
impl ClientActor for Standard {
    async fn get_uuid(
        &self,
        username: &str,
        backend: &dyn ClientBackend,
    ) -> Result<Uuid, ActorError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Fetching UUID for: \"{username}\"");

        let response = backend
            .get(
                &format!("https://api.mojang.com/users/profiles/minecraft/{username}"),
                Some(&[("Accept", "application/json")]),
            )
            .await?;

        serde_json::from_slice::<NameAndUuid>(&response)
            .map_or_else(|err| Err(ActorError::Serde(err)), |data| Ok(data.id))
    }

    async fn get_username(
        &self,
        uuid: Uuid,
        backend: &dyn ClientBackend,
    ) -> Result<String, ActorError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Fetching username for: \"{uuid}\"");

        let response = backend.get(
            &format!(
                "https://sessionserver.mojang.com/session/minecraft/profile/{uuid}?unsigned=false"
            ),
            Some(&[("Accept", "application/json")]),
        ).await?;

        serde_json::from_slice::<NameAndUuid>(&response)
            .map_or_else(|err| Err(ActorError::Serde(err)), |data| Ok(data.name))
    }

    async fn get_profile(
        &self,
        uuid: Uuid,
        backend: &dyn ClientBackend,
    ) -> Result<PlayerProfile, ActorError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Fetching profile for: \"{uuid}\"");

        let response = backend
            .get(
                &format!("https://sessionserver.mojang.com/session/minecraft/profile/{uuid}"),
                Some(&[("Accept", "application/json")]),
            )
            .await?;

        serde_json::from_slice::<RawProfile>(&response)
            .map_or_else(|err| Err(ActorError::Serde(err)), |data| Ok(PlayerProfile::from(data)))
    }

    async fn api_public_keys(
        &self,
        backend: &dyn ClientBackend,
    ) -> Result<ApiPublicKeys, ActorError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Fetching API public keys");

        let response = backend
            .get(
                "https://api.minecraftservices.com/publickeys",
                Some(&[("Accept", "application/json")]),
            )
            .await?;

        serde_json::from_slice::<RawPublicKeys>(&response).map_or_else(
            |err| Err(ActorError::Serde(err)),
            |data| ApiPublicKeys::try_from(data).map_err(|err| ActorError::Other(Box::new(err))),
        )
    }

    async fn notify_login(
        &self,
        token: &str,
        uuid: Uuid,
        server_id: &ServerId,
        backend: &dyn ClientBackend,
    ) -> Result<(), ActorError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Notifying API of login for \"{uuid}\" with server ID: \"{server_id}\"");

        backend
            .post(
                "https://sessionserver.mojang.com/session/minecraft/join",
                Some(&[("Content-Type", "application/json")]),
                &serde_json::to_vec(&NotifyLoginPayload { token, uuid, server_id })?,
            )
            .await
            .map_or_else(|err| Err(ActorError::Backend(err)), |_| Ok(()))
    }

    async fn verify_login(
        &self,
        username: &str,
        server_id: &ServerId,
        client_ip: Option<IpAddr>,
        backend: &dyn ClientBackend,
    ) -> Result<Option<PlayerProfile>, ActorError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Verifying login for \"{username}\" with server ID: \"{server_id}\"");

        let response = match client_ip {
            Some(client_ip) => backend.get(
                &format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={username}&serverId={server_id}&ip={client_ip}"),
                Some(&[("Accept", "application/json")]),
            ).await?,
            None => backend.get(
                &format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={username}&serverId={server_id}"),
                Some(&[("Accept", "application/json")]),
            ).await?,
        };

        match serde_json::from_slice::<RawProfile>(&response) {
            Ok(data) => Ok(Some(PlayerProfile::from(data))),
            Err(..) => Ok(None),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A response containing a player's name and UUID.
///
/// # Example
///
/// ```json
/// {
///     "name": "jeb_",
///     "id": "853c80ef3c3749fdaa49938b674adae6"
/// }
/// ```
#[derive(Deserialize)]
struct NameAndUuid {
    id: Uuid,
    name: String,
}

// -------------------------------------------------------------------------------------------------

/// A response containing a player's profile data.
///
/// # Example
///
/// ```json
/// {
///     "id": "853c80ef3c3749fdaa49938b674adae6",
///     "name": "jeb_",
///     "properties": [
///         {
///             "name": "textures",
///             "value": "ewogICJ0aW1lc3R..."
///         }
///     ]
/// }
/// ```
#[derive(Deserialize)]
struct RawProfile {
    id: Uuid,
    name: String,
    properties: Vec<RawProfileProperty>,
}

#[derive(Debug, Deserialize)]
struct RawProfileProperty {
    name: String,
    value: String,
}

impl From<RawProfile> for PlayerProfile {
    fn from(value: RawProfile) -> Self {
        let mut properties = IndexMap::with_hasher(RandomState::default());
        let mut skin = None;
        let mut cape = None;

        for prop in value.properties {
            let Ok(decoded) = BASE64_STANDARD.decode(prop.value) else {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to decode profile property \"{}\"", prop.name);

                continue;
            };

            let Ok(deserialized) = serde_json::from_slice::<serde_json::Value>(decoded.as_slice())
            else {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to parse profile property \"{}\"", prop.name);
                continue;
            };

            if prop.name == "textures"
                && let serde_json::Value::Object(textures) = &deserialized
                && let Some(serde_json::Value::Object(textures)) = textures.get("textures")
            {
                if let Some(serde_json::Value::Object(value)) = textures.get("SKIN")
                    && let Some(serde_json::Value::String(value)) = value.get("url")
                {
                    skin = Some(value.clone());
                }

                if let Some(serde_json::Value::Object(value)) = textures.get("CAPE")
                    && let Some(serde_json::Value::String(value)) = value.get("url")
                {
                    cape = Some(value.clone());
                }
            }

            properties.insert(prop.name, deserialized.into());
        }

        PlayerProfile { username: value.name, uuid: value.id, skin, cape, properties }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Deserialize)]
struct RawPublicKeys {
    #[serde(rename = "profilePropertyKeys")]
    profile_keys: Vec<PublicKeyObject>,
    #[serde(rename = "playerCertificateKeys")]
    certificate_keys: Vec<PublicKeyObject>,
}

#[derive(Deserialize)]
struct PublicKeyObject {
    #[serde(rename = "publicKey")]
    public_key: String,
}

impl TryFrom<RawPublicKeys> for ApiPublicKeys {
    type Error = ActorError;

    fn try_from(value: RawPublicKeys) -> Result<Self, Self::Error> {
        let mut profile = Vec::with_capacity(value.profile_keys.len());
        for key in value.profile_keys {
            let der = BASE64_STANDARD
                .decode(key.public_key)
                .map_err(|_| ActorError::Rsa(rsa::Error::Internal))?;
            let key = RsaPublicKey::from_public_key_der(&der)?;

            profile.push(key);
        }

        let mut certificate = Vec::with_capacity(value.certificate_keys.len());
        for key in value.certificate_keys {
            let der = BASE64_STANDARD
                .decode(key.public_key)
                .map_err(|_| ActorError::Rsa(rsa::Error::Internal))?;
            let key = RsaPublicKey::from_public_key_der(&der)?;

            certificate.push(key);
        }

        Ok(ApiPublicKeys { profile, certificate })
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Serialize)]
struct NotifyLoginPayload<'a> {
    #[serde(rename = "accessToken")]
    token: &'a str,
    #[serde(rename = "selectedProfile")]
    uuid: Uuid,
    #[serde(rename = "serverId")]
    server_id: &'a str,
}
