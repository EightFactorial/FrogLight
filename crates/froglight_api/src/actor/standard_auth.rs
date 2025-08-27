//! TODO

use alloc::{boxed::Box, string::String};
use core::net::IpAddr;

use uuid::Uuid;

use crate::{
    actor::{
        ActorError, ClientActor, ClientAuthActor, PlayerProfile,
        standard::Standard,
        types::{ApiPublicKeys, ServerId},
    },
    backend::ClientBackend,
};

/// The standard [`ApiAuthClient`](crate::client::ApiAuthClient) actor.
///
/// Uses Mojang's API and [`serde`] for deserialization.
pub struct StandardAuth();

#[async_trait::async_trait]
impl ClientActor for StandardAuth {
    #[inline]
    async fn get_uuid(
        &self,
        username: &str,
        backend: &dyn ClientBackend,
    ) -> Result<Uuid, ActorError> {
        Standard.get_uuid(username, backend).await
    }

    #[inline]
    async fn get_username(
        &self,
        uuid: Uuid,
        backend: &dyn ClientBackend,
    ) -> Result<String, ActorError> {
        Standard.get_username(uuid, backend).await
    }

    #[inline]
    async fn get_profile(
        &self,
        uuid: Uuid,
        backend: &dyn ClientBackend,
    ) -> Result<PlayerProfile, ActorError> {
        Standard.get_profile(uuid, backend).await
    }

    #[inline]
    async fn api_public_keys(
        &self,
        backend: &dyn ClientBackend,
    ) -> Result<ApiPublicKeys, ActorError> {
        Standard.api_public_keys(backend).await
    }

    #[inline]
    async fn notify_login(
        &self,
        token: &str,
        uuid: Uuid,
        server_id: &ServerId,
        backend: &dyn ClientBackend,
    ) -> Result<(), ActorError> {
        Standard.notify_login(token, uuid, server_id, backend).await
    }

    #[inline]
    async fn verify_login(
        &self,
        username: &str,
        server_id: &ServerId,
        client_ip: Option<IpAddr>,
        backend: &dyn ClientBackend,
    ) -> Result<Option<PlayerProfile>, ActorError> {
        Standard.verify_login(username, server_id, client_ip, backend).await
    }
}

// -------------------------------------------------------------------------------------------------

impl ClientAuthActor for StandardAuth {}
