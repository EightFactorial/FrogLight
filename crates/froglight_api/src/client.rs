//! TODO

use alloc::{string::String, sync::Arc};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use uuid::Uuid;

use crate::{
    actor::{
        ActorError, ApiPublicKeys, ClientActor, ClientAuthActor, PlayerProfile, standard::Standard,
    },
    backend::ClientBackend,
    builder::ApiClientBuilder,
};

/// A client for making API requests.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect), reflect(Clone, Resource, opaque))]
pub struct ApiClient {
    actor: Arc<dyn ClientActor>,
    backend: Arc<dyn ClientBackend>,
}

impl ApiClient {
    /// Create a new [`ApiClientBuilder`] to build an [`ApiClient`].
    #[inline]
    #[must_use]
    pub const fn builder() -> ApiClientBuilder<(), ()> { ApiClientBuilder::new() }

    /// Create a new [`ApiClient`] from the given [`ClientActor`] and
    /// [`ClientBackend`].
    #[inline]
    #[must_use]
    pub(crate) fn new(actor: impl ClientActor, backend: impl ClientBackend) -> Self {
        Self { actor: Arc::new(actor), backend: Arc::new(backend) }
    }

    /// Create a new [`ApiClient`] using the [`Standard`] actor and the given
    /// [`ClientBackend`].
    #[inline]
    #[must_use]
    pub fn standard_with(backend: impl ClientBackend) -> ApiClient {
        ApiClient::builder().with_actor(Standard).with_backend(backend).build()
    }

    /// Create a new [`ApiClient`] using the [`Standard`] actor and the
    /// [`froglight_network::resolver::HttpClient`] backend.
    #[must_use]
    #[cfg(all(feature = "resolver", feature = "ureq"))]
    pub fn standard() -> ApiClient {
        let agent = &**froglight_network::prelude::HttpClient::get_or_default();
        ApiClient::standard_with(agent.clone())
    }

    /// Get a reference to the [`ClientActor`] for this [`ApiClient`].
    #[inline]
    #[must_use]
    pub fn actor(&self) -> &dyn ClientActor { self.actor.as_ref() }

    /// Get a reference to the [`ClientBackend`] for this [`ApiClient`].
    #[inline]
    #[must_use]
    pub fn backend(&self) -> &dyn ClientBackend { self.backend.as_ref() }

    /// Get the API's public keys for verifying signatures.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn get_public_keys(&self) -> Result<ApiPublicKeys, ActorError> {
        self.actor.api_public_keys(self.backend.as_ref()).await
    }

    /// Get a player's UUID from their username.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn get_uuid(&self, username: &str) -> Result<Uuid, ActorError> {
        self.actor.get_uuid(username, self.backend.as_ref()).await
    }

    /// Get a player's username from their UUID.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn get_username(&self, uuid: Uuid) -> Result<String, ActorError> {
        self.actor.get_username(uuid, self.backend.as_ref()).await
    }

    /// Get a player's [`PlayerProfile`] from their UUID.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn get_profile(&self, uuid: Uuid) -> Result<PlayerProfile, ActorError> {
        self.actor.get_profile(uuid, self.backend.as_ref()).await
    }
}

// -------------------------------------------------------------------------------------------------

/// A client for making authenticated API requests.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect), reflect(Clone, Resource, opaque))]
pub struct ApiAuthClient {
    actor: Arc<dyn ClientAuthActor>,
    backend: Arc<dyn ClientBackend>,
}

impl ApiAuthClient {
    /// Create a new [`ApiClientBuilder`] to build an [`ApiAuthClient`].
    #[inline]
    #[must_use]
    pub const fn builder() -> ApiClientBuilder<(), ()> { ApiClientBuilder::new() }

    /// Create a new [`ApiAuthClient`] from the given [`ClientActor`] and
    /// [`ClientBackend`].
    #[inline]
    #[must_use]
    pub(crate) fn new(actor: impl ClientAuthActor, backend: impl ClientBackend) -> Self {
        Self { actor: Arc::new(actor), backend: Arc::new(backend) }
    }

    /// Get a reference to the [`ClientAuthActor`] for this [`ApiAuthClient`].
    #[inline]
    #[must_use]
    pub fn actor(&self) -> &dyn ClientAuthActor { self.actor.as_ref() }

    /// Get a reference to the [`ClientBackend`] for this [`ApiAuthClient`].
    #[inline]
    #[must_use]
    pub fn backend(&self) -> &dyn ClientBackend { self.backend.as_ref() }

    /// Get the API's public keys for verifying signatures.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn get_public_keys(&self) -> Result<ApiPublicKeys, ActorError> {
        self.actor.api_public_keys(self.backend.as_ref()).await
    }

    /// Get a player's UUID from their username.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn get_uuid(&self, username: &str) -> Result<Uuid, ActorError> {
        self.actor.get_uuid(username, self.backend.as_ref()).await
    }

    /// Get a player's username from their UUID.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn get_username(&self, uuid: Uuid) -> Result<String, ActorError> {
        self.actor.get_username(uuid, self.backend.as_ref()).await
    }

    /// Get a player's [`PlayerProfile`] from their UUID.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn get_profile(&self, uuid: Uuid) -> Result<PlayerProfile, ActorError> {
        self.actor.get_profile(uuid, self.backend.as_ref()).await
    }
}
