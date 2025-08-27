//! TODO

use crate::{
    actor::{ClientActor, ClientAuthActor},
    backend::ClientBackend,
    client::{ApiAuthClient, ApiClient},
};

/// A builder for creating an [`ApiClient`].
pub struct ApiClientBuilder<A, B> {
    actor: A,
    backend: B,
}

impl Default for ApiClientBuilder<(), ()> {
    fn default() -> Self { Self::new() }
}

impl ApiClientBuilder<(), ()> {
    /// Create a new, empty [`ApiClientBuilder`].
    #[must_use]
    pub const fn new() -> Self { Self { actor: (), backend: () } }
}

impl<A1, B1> ApiClientBuilder<A1, B1> {
    /// Get a reference to the [`ClientActor`] for this [`ApiClientBuilder`].
    #[inline]
    #[must_use]
    pub const fn actor(&self) -> &A1 { &self.actor }

    /// Get a mutable reference to the [`ClientActor`] for this
    /// [`ApiClientBuilder`].
    #[inline]
    #[must_use]
    pub const fn actor_mut(&mut self) -> &mut A1 { &mut self.actor }

    /// Set the [`ClientActor`] for this [`ApiClientBuilder`].
    #[inline]
    #[must_use]
    pub fn with_actor<A2: ClientActor>(self, actor: A2) -> ApiClientBuilder<A2, B1> {
        ApiClientBuilder { actor, backend: self.backend }
    }

    /// Set the [`ClientActor`] for this [`ApiClientBuilder`].
    #[inline]
    #[must_use]
    pub fn with_authed_actor<A2: ClientAuthActor>(self, actor: A2) -> ApiClientBuilder<A2, B1> {
        ApiClientBuilder { actor, backend: self.backend }
    }

    /// Get a reference to the [`ClientBackend`] for this [`ApiClientBuilder`].
    #[inline]
    #[must_use]
    pub const fn backend(&self) -> &B1 { &self.backend }

    /// Get a mutable reference to the [`ClientBackend`] for this
    /// [`ApiClientBuilder`].
    #[inline]
    #[must_use]
    pub const fn backend_mut(&mut self) -> &mut B1 { &mut self.backend }

    /// Set the [`ClientBackend`] for this [`ApiClientBuilder`].
    #[inline]
    #[must_use]
    pub fn with_backend<B2: ClientBackend>(self, backend: B2) -> ApiClientBuilder<A1, B2> {
        ApiClientBuilder { actor: self.actor, backend }
    }
}

impl<A: ClientActor, B: ClientBackend> ApiClientBuilder<A, B> {
    /// Build an [`ApiClient`] from this builder.
    #[inline]
    #[must_use]
    pub fn build(self) -> ApiClient { ApiClient::new(self.actor, self.backend) }
}

impl<A: ClientAuthActor, B: ClientBackend> ApiClientBuilder<A, B> {
    /// Build an [`ApiAuthClient`] from this builder.
    #[inline]
    #[must_use]
    pub fn build_authed(self) -> ApiAuthClient { ApiAuthClient::new(self.actor, self.backend) }
}
