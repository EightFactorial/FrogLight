//! TODO

use alloc::{boxed::Box, vec::Vec};
use core::{
    error::Error,
    fmt::{Debug, Display},
};

#[cfg(feature = "ureq")]
mod ureq;

/// A client backend for making API requests.
#[async_trait::async_trait]
pub trait ClientBackend: Send + Sync + 'static {
    /// Send a `GET` request to the given URL and return the response body.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be sent
    /// or if the server returns an error.
    async fn get(
        &self,
        url: &str,
        headers: Option<&[(&str, &str)]>,
    ) -> Result<Vec<u8>, BackendError>;

    /// Send a `POST` request to the given URL with the provided body
    /// and return the response.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be sent
    /// or if the server returns an error.
    async fn post(
        &self,
        url: &str,
        headers: Option<&[(&str, &str)]>,
        body: &[u8],
    ) -> Result<Vec<u8>, BackendError>;

    /// Send a `PUT` request to the given URL with the provided body
    /// and return the response.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be sent
    /// or if the server returns an error.
    async fn put(
        &self,
        url: &str,
        headers: Option<&[(&str, &str)]>,
        body: &[u8],
    ) -> Result<Vec<u8>, BackendError>;

    /// Send a `DELETE` request to the given URL with an optional body and
    /// return the response.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be sent
    /// or if the server returns an error.
    async fn delete(
        &self,
        url: &str,
        headers: Option<&[(&str, &str)]>,
        body: Option<&[u8]>,
    ) -> Result<Vec<u8>, BackendError>;
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur when interacting with the backend.
pub enum BackendError {
    /// The server returned an unexpected status code.
    StatusCode(u16),
    /// A hostname could not be resolved.
    HostNotFound,
    /// An IO error occurred.
    #[cfg(feature = "std")]
    Io(std::io::Error),
    /// Some other error occurred.
    Other(Box<dyn Error + Sync + Send>),
}

impl Error for BackendError {}
impl Debug for BackendError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StatusCode(arg0) => f.debug_tuple("StatusCode").field(arg0).finish(),
            Self::HostNotFound => write!(f, "HostNotFound"),
            #[cfg(feature = "std")]
            Self::Io(arg0) => f.debug_tuple("Io").field(arg0).finish(),
            Self::Other(arg0) => f.debug_tuple("Other").field(arg0).finish(),
        }
    }
}
impl Display for BackendError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StatusCode(arg0) => write!(f, "Status code: {arg0}"),
            Self::HostNotFound => write!(f, "Host not found"),
            #[cfg(feature = "std")]
            Self::Io(arg0) => write!(f, "IO error: {arg0}"),
            Self::Other(arg0) => write!(f, "Other error: {arg0}"),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for BackendError {
    fn from(err: std::io::Error) -> Self { Self::Io(err) }
}
