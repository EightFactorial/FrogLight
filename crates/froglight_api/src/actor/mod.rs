//! TODO

use alloc::{boxed::Box, string::String};
use core::{
    error::Error,
    fmt::{Debug, Display},
    net::IpAddr,
    result::Result,
};

use uuid::Uuid;

mod types;
pub use types::*;

pub mod standard;
pub mod standard_auth;

use crate::backend::{BackendError, ClientBackend};

/// A client actor for making unauthenticated API requests.
#[async_trait::async_trait]
pub trait ClientActor: Send + Sync + 'static {
    /// Retrieve the UUID of a player by their username.
    ///
    /// # Errors
    ///
    /// Returns an error if the server returns an unexpected response or an
    /// error.
    async fn get_uuid(
        &self,
        username: &str,
        backend: &dyn ClientBackend,
    ) -> Result<Uuid, ActorError>;
    /// Retrieve the username of a player by their UUID.
    ///
    /// # Errors
    ///
    /// Returns an error if the server returns an unexpected response or an
    /// error.
    async fn get_username(
        &self,
        uuid: Uuid,
        backend: &dyn ClientBackend,
    ) -> Result<String, ActorError>;
    /// Retrieve a player's profile by their UUID.
    ///
    /// # Errors
    ///
    /// Returns an error if the server returns an unexpected response or an
    /// error.
    async fn get_profile(
        &self,
        uuid: Uuid,
        backend: &dyn ClientBackend,
    ) -> Result<PlayerProfile, ActorError>;

    /// Retrieve the API's public keys for verifying signatures.
    ///
    /// # Errors
    ///
    /// Returns an error if the server returns an unexpected response or an
    /// error.
    async fn api_public_keys(
        &self,
        backend: &dyn ClientBackend,
    ) -> Result<ApiPublicKeys, ActorError>;

    /// Inform the API that the player has logged in to a server.
    ///
    /// Performed by clients when connecting to a server.
    ///
    /// # Errors
    ///
    /// Returns an error if the server returns an unexpected response or an
    /// error.
    async fn notify_login(
        &self,
        token: &str,
        uuid: Uuid,
        server_id: &ServerId,
        backend: &dyn ClientBackend,
    ) -> Result<(), ActorError>;
    /// Request the API to authenticate a login attempt.
    ///
    /// Performed by servers when a player attempts to join.
    ///
    /// # Errors
    ///
    /// Returns an error if the server returns an unexpected response or an
    /// error.
    async fn verify_login(
        &self,
        username: &str,
        server_id: &ServerId,
        client_ip: Option<IpAddr>,
        backend: &dyn ClientBackend,
    ) -> Result<Option<PlayerProfile>, ActorError>;
}

/// A client actor for making authenticated API requests.
#[async_trait::async_trait]
pub trait ClientAuthActor: ClientActor {}

// -------------------------------------------------------------------------------------------------

/// An error that can occur when interacting with the actor.
pub enum ActorError {
    /// An error that occurred while interacting with the backend.
    Backend(BackendError),
    /// An error that occurred while deserializing data.
    Serde(serde_json::Error),
    /// An error that occurred while handling RSA keys.
    Rsa(rsa::Error),
    /// An RSA SPKI error occurred.
    Spki(rsa::pkcs8::spki::Error),
    /// Some other error occurred.
    Other(Box<dyn Error + Sync + Send>),
}

impl Error for ActorError {}
impl Debug for ActorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Backend(arg0) => f.debug_tuple("Backend").field(arg0).finish(),
            Self::Serde(arg0) => f.debug_tuple("Serde").field(arg0).finish(),
            Self::Rsa(arg0) => f.debug_tuple("Rsa").field(arg0).finish(),
            Self::Spki(arg0) => f.debug_tuple("Spki").field(arg0).finish(),
            Self::Other(arg0) => f.debug_tuple("Other").field(arg0).finish(),
        }
    }
}
impl Display for ActorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Backend(arg0) => write!(f, "Backend error: {arg0}"),
            Self::Serde(arg0) => write!(f, "Serde error: {arg0}"),
            Self::Rsa(arg0) => write!(f, "RSA error: {arg0}"),
            Self::Spki(arg0) => write!(f, "SPKI error: {arg0}"),
            Self::Other(arg0) => write!(f, "Other error: {arg0}"),
        }
    }
}

impl From<BackendError> for ActorError {
    fn from(err: BackendError) -> Self { Self::Backend(err) }
}
impl From<rsa::Error> for ActorError {
    fn from(err: rsa::Error) -> Self { Self::Rsa(err) }
}
impl From<rsa::pkcs8::spki::Error> for ActorError {
    fn from(err: rsa::pkcs8::spki::Error) -> Self { Self::Spki(err) }
}
impl From<serde_json::Error> for ActorError {
    fn from(err: serde_json::Error) -> Self { Self::Serde(err) }
}
