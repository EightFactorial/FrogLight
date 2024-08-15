use froglight_protocol::{common::ResourceKey, traits::Version};

/// A trait for converting between keys and a registry values.
pub trait ConvertKey
where
    Self: Sized + Send + Sync,
{
    /// Convert a [`ResourceKey`] to a registry value.
    ///
    /// Returns `None` if the key does not match any known value.
    fn from_key(key: &str) -> Option<Self>;

    /// Convert the registry value to a [`ResourceKey`].
    fn to_key(&self) -> &'static ResourceKey;
}

/// A trait for converting between IDs and a registry values.
pub trait ConvertId<V: Version>
where
    Self: Sized + Send + Sync,
{
    /// Convert an ID to a registry value.
    ///
    /// Returns `None` if the ID does not match any known value.
    fn from_id(id: u32) -> Option<Self>;

    /// Convert the registry value to an ID.
    /// 
    /// Returns `None` if the value does not match any known id.
    fn to_id(&self) -> Option<u32>;
}
