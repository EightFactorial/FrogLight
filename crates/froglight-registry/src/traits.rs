use froglight_protocol::traits::Version;

/// A trait for converting between a registry key and its ID.
pub trait RegistryId<V: Version>: Sized {
    /// Get the ID of the registry value.
    fn as_id(&self) -> Option<u32>;

    /// Get the registry value from the ID.
    fn from_id(id: u32) -> Option<Self>;
}

/// A trait for converting between a registry key and its value.
pub trait RegistryKey<V: Version>: Sized {
    /// Get the key of the registry value.
    fn as_key(&self) -> Option<&'static str>;

    /// Get the registry value from the key.
    fn from_key(key: &str) -> Option<Self>;
}
