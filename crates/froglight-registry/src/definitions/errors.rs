//! Errors that occur while using registries.

use froglight_protocol::common::{ResourceKey, ResourceKeyError};

/// An error that occurred while converting between a registry value and a key.
#[derive(Debug, thiserror::Error)]
pub enum ConvertKeyError<E>
where
    E: std::error::Error,
{
    /// A resource key error occurred.
    #[error(transparent)]
    ResourceKey(#[from] ResourceKeyError),
    /// A conversion error occurred.
    #[error(transparent)]
    Other(E),
}

/// There is no value for the specified
/// [`key`](super::MissingKeyError).
#[derive(Debug, thiserror::Error)]
#[error("There is no value for the specified key: {key}")]
pub struct MissingKeyError {
    /// The key that is missing.
    pub key: ResourceKey,
}

impl From<ResourceKey> for MissingKeyError {
    fn from(key: ResourceKey) -> Self { Self { key } }
}
