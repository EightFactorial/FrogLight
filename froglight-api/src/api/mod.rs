//! TODO

use std::{
    error::Error,
    fmt::{self, Debug, Display},
    str::Utf8Error,
    string::FromUtf8Error,
    sync::Arc,
};

use async_trait::async_trait;
#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use uuid::Uuid;

use crate::client::{HttpClient, HttpError};

mod mojang;
pub use mojang::Mojang;

/// The client's API interface.
///
/// Uses the [`Mojang`] API by default.
///
/// ## Note
///
/// This type is thread-safe and can be cloned cheaply.
#[repr(transparent)]
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Default, Clone, Component))]
pub struct ClientApi(Arc<dyn NetworkApi>);

impl Default for ClientApi {
    fn default() -> Self { Self::new(Mojang) }
}

impl ClientApi {
    /// Creates a new [`ClientApi`] from a [`NetworkApi`].
    #[inline]
    #[must_use]
    pub fn new<T: NetworkApi>(agent: T) -> Self { Self::new_arc(Arc::new(agent)) }

    /// Creates a new [`ClientApi`] from an [`Arc<dyn NetworkApi>`].
    #[inline]
    #[must_use]
    pub const fn new_arc(agent: Arc<dyn NetworkApi>) -> Self { Self(agent) }

    /// Returns a reference to the inner [`Arc<dyn NetworkApi>`].
    #[inline]
    #[must_use]
    pub const fn as_arc(&self) -> &Arc<dyn NetworkApi> { &self.0 }

    /// Queries the API for the UUID of a given username.
    ///
    /// Returns `None` if no account with the given username exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    pub async fn query_uuid(
        &self,
        username: &str,
        client: &HttpClient,
    ) -> Result<Option<Uuid>, ApiError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_api::api", "Querying uuid of \"{username}\"");
        self.0.query_uuid(username, client).await
    }

    /// Queries the API for the username of a given UUID.
    ///
    /// Returns `None` if no account with the given UUID exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    pub async fn query_username(
        &self,
        uuid: Uuid,
        client: &HttpClient,
    ) -> Result<Option<String>, ApiError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_api::api", "Querying username of \"{uuid}\"");
        self.0.query_username(uuid, client).await
    }
}

impl Debug for ClientApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ClientApi").field(&"Arc<dyn NetworkApi>").finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for types that can perform api calls.
#[async_trait]
pub trait NetworkApi: Send + Sync + 'static {
    /// Queries the API for the UUID of a given username.
    ///
    /// Returns `None` if no account with the given username exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn query_uuid(
        &self,
        username: &str,
        client: &HttpClient,
    ) -> Result<Option<Uuid>, ApiError>;

    /// Queries the API for the username of a given UUID.
    ///
    /// Returns `None` if no account with the given UUID exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn query_username(
        &self,
        uuid: Uuid,
        client: &HttpClient,
    ) -> Result<Option<String>, ApiError>;
}

/// An error that occurred while performing an API call.
#[derive(Debug)]
pub enum ApiError {
    /// An HTTP error occurred.
    Http(HttpError),
    /// A UTF-8 error occurred.
    Utf8(Utf8Error),
    /// A JSON error occurred.
    Serde(serde_json::Error),

    /// An unspecified error occurred.
    Other(Box<dyn Error + Send + Sync>),
}

impl Error for ApiError {}
impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Http(err) => write!(f, "failed to make request, {err}"),
            ApiError::Utf8(err) => write!(f, "failed to parse response as utf-8, {err}"),
            ApiError::Serde(err) => write!(f, "failed to parse response as json, {err}"),
            ApiError::Other(err) => Display::fmt(err, f),
        }
    }
}

impl From<HttpError> for ApiError {
    fn from(value: HttpError) -> Self { ApiError::Http(value) }
}

impl From<Utf8Error> for ApiError {
    fn from(value: Utf8Error) -> Self { ApiError::Utf8(value) }
}
impl From<FromUtf8Error> for ApiError {
    fn from(value: FromUtf8Error) -> Self { ApiError::Utf8(value.utf8_error()) }
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self { ApiError::Serde(value) }
}
