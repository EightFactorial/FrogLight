use froglight_protocol::common::ResourceKey;

use super::errors::ConvertKeyError;

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
