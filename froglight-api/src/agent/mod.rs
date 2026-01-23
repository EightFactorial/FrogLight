//! TODO

use std::{
    error::Error,
    fmt::{self, Display},
    ops::Deref,
    sync::Arc,
};

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
pub use ::reqwest::blocking::Client as ReqwestAgent;

/// An agent which can perform network operations.
#[repr(transparent)]
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Clone, Resource))]
pub struct Agent(Arc<dyn NetworkAgent>);

impl Agent {
    /// Creates a new [`Agent`] from a [`NetworkAgent`].
    #[inline]
    #[must_use]
    pub fn new<T: NetworkAgent>(agent: T) -> Self { Self::new_arc(Arc::new(agent)) }

    /// Creates a new [`Agent`] from an [`Arc<dyn NetworkAgent>`].
    #[inline]
    #[must_use]
    pub const fn new_arc(agent: Arc<dyn NetworkAgent>) -> Self { Self(agent) }

    /// Returns a reference to the inner [`Arc<dyn NetworkAgent>`].
    #[inline]
    #[must_use]
    pub const fn as_arc(&self) -> &Arc<dyn NetworkAgent> { &self.0 }
}

impl AsRef<dyn NetworkAgent> for Agent {
    #[inline]
    fn as_ref(&self) -> &dyn NetworkAgent { &*self.0 }
}

impl Deref for Agent {
    type Target = dyn NetworkAgent;

    #[inline]
    fn deref(&self) -> &Self::Target { &*self.0 }
}

// -------------------------------------------------------------------------------------------------

/// A trait for types that can act as network agents.
pub trait NetworkAgent: Send + Sync + 'static {
    /// Performs a `GET` request to the specified URL.
    ///
    /// # Errors
    ///
    /// Returns an [`AgentError`] if the request fails.
    fn get(&self, url: &str, opts: GetOptions) -> Result<GetResponse, AgentError>;
    /// Performs a `PUT` request to the specified URL.
    ///
    /// # Errors
    ///
    /// Returns an [`AgentError`] if the request fails.
    fn put(&self, url: &str, data: Vec<u8>, opts: PutOptions) -> Result<PutResponse, AgentError>;
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

/// An error that can occur when using a [`NetworkAgent`].
#[derive(Debug)]
pub enum AgentError {
    /// An HTTP error occurred with the given status code.
    Http(u16),
    /// An unspecified error occurred.
    Other(Box<dyn Error + Send + Sync>),
}

impl Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::Http(code) => write!(f, "HTTP error {code}"),
            AgentError::Other(err) => Display::fmt(err, f),
        }
    }
}
