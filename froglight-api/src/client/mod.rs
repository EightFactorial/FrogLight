//! TODO

use std::{
    error::Error,
    fmt::{self, Debug, Display},
    sync::Arc,
};

use async_trait::async_trait;
#[cfg(feature = "bevy")]
use bevy_ecs::{reflect::ReflectResource, resource::Resource};
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;

#[cfg(feature = "ureq")]
mod ureq;
#[cfg(feature = "ureq")]
pub use ::ureq::Agent as UreqAgent;

#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(feature = "reqwest")]
pub use ::reqwest::Client as ReqwestAgent;

/// A client which can perform network operations.
///
/// Supports multiple underlying implementations via the [`NetworkClient`]
/// trait.
///
/// ## Note
///
/// This type is thread-safe and can be cloned cheaply.
#[repr(transparent)]
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, Resource))]
pub struct HttpClient(Arc<dyn NetworkClient>);

impl HttpClient {
    /// Creates a new [`HttpClient`] from a [`NetworkClient`].
    #[inline]
    #[must_use]
    pub fn new<T: NetworkClient>(agent: T) -> Self { Self::new_arc(Arc::new(agent)) }

    /// Creates a new [`HttpClient`] from an [`Arc<dyn NetworkClient>`].
    #[inline]
    #[must_use]
    pub const fn new_arc(agent: Arc<dyn NetworkClient>) -> Self { Self(agent) }

    /// Returns a reference to the inner [`Arc<dyn NetworkClient>`].
    #[inline]
    #[must_use]
    pub const fn as_arc(&self) -> &Arc<dyn NetworkClient> { &self.0 }

    /// Performs a `GET` request to the specified URL.
    ///
    /// # Errors
    ///
    /// Returns an [`HttpError`] if the request fails.
    pub async fn get(&self, url: &str, opts: GetOptions) -> Result<GetResponse, HttpError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_api::client", "GET \"{url}\"");
        self.0.get(url, opts).await
    }

    /// Performs a `PUT` request to the specified URL.
    ///
    /// # Errors
    ///
    /// Returns an [`HttpError`] if the request fails.
    pub async fn put(
        &self,
        url: &str,
        data: Vec<u8>,
        opts: PutOptions,
    ) -> Result<PutResponse, HttpError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_api::client", "PUT \"{url}\"");
        self.0.put(url, data, opts).await
    }
}

impl Debug for HttpClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("HttpClient").field(&"Arc<dyn NetworkClient>").finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for types that can act as a network client.
#[async_trait]
pub trait NetworkClient: Send + Sync + 'static {
    /// Performs a `GET` request to the specified URL.
    ///
    /// # Errors
    ///
    /// Returns an [`HttpError`] if the request fails.
    async fn get(&self, url: &str, opts: GetOptions) -> Result<GetResponse, HttpError>;
    /// Performs a `PUT` request to the specified URL.
    ///
    /// # Errors
    ///
    /// Returns an [`HttpError`] if the request fails.
    async fn put(
        &self,
        url: &str,
        data: Vec<u8>,
        opts: PutOptions,
    ) -> Result<PutResponse, HttpError>;
}

/// Options for a `GET` request.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GetOptions {}

/// The response from a `GET` request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetResponse {
    /// The HTTP status code.
    pub status: u16,
    /// The response data.
    pub data: Vec<u8>,
    /// The response headers.
    pub headers: Vec<(String, Vec<u8>)>,
}

/// Options for a `PUT` request.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PutOptions {}

/// The response from a `PUT` request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutResponse {
    /// The HTTP status code.
    pub status: u16,
}

/// An error that can occur when using a [`NetworkClient`].
#[derive(Debug)]
pub enum HttpError {
    /// An HTTP error occurred with the given status code.
    Http(u16),
    /// An unspecified error occurred.
    Other(Box<dyn Error + Send + Sync>),
}

impl Error for HttpError {}
impl Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::Http(code) => write!(f, "HTTP error {code}"),
            HttpError::Other(err) => Display::fmt(err, f),
        }
    }
}
