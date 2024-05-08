use froglight_protocol::common::ResourceKey;

/// A trait for converting between keys and a registry values.
pub trait ConvertKey
where
    Self: Sized + Send + Sync,
{
    /// The type of error that can occur while converting a key
    /// to a registry value.
    type Error: std::error::Error;

    /// Convert a [`ResourceKey`] to a registry value.
    ///
    /// # Errors
    /// Returns an error if the key does not match any registry value.
    fn from_key(key: &(impl AsRef<str> + ?Sized)) -> Result<Self, Self::Error>;

    /// Get the key as a [`str`].
    fn to_key_str(&self) -> &str;

    /// Convert the registry value to a [`ResourceKey`].
    fn to_key(&self) -> ResourceKey { ResourceKey::new(self.to_key_str()) }
}
