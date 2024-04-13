use froglight_protocol::common::{ResourceKey, ResourceKeyError};

/// A trait for converting between keys and a registry values.
pub trait ConvertKey
where
    Self: Sized + Send + Sync,
{
    /// The type of error that can occur while converting a key
    /// to a registry value.
    type Error: std::error::Error;

    /// Convert a [`ResourceKey`] to a registry value.
    #[allow(clippy::missing_errors_doc)]
    fn from_key(key: &ResourceKey) -> Result<Self, Self::Error>;

    /// Convert a key to a registry value.
    ///
    /// # Errors
    /// If the key is not a valid [`ResourceKey`].
    fn try_from_key(
        key: &(impl AsRef<str> + ?Sized),
    ) -> Result<Self, ConvertKeyError<Self::Error>> {
        Self::from_key(&ResourceKey::try_new(key.as_ref())?).map_err(ConvertKeyError::Other)
    }

    /// Convert the registry value to a [`ResourceKey`].
    fn to_key(&self) -> ResourceKey;
}

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
