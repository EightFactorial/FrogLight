use std::any::Any;

use crate::{common::ResourceKey, traits::Version};

/// A trait for converting between IDs and a registry values.
pub trait ConvertId<V>
where
    Self: Sized,
    V: Version,
{
    /// Convert the ID to a registry value.
    fn from_id(id: u32) -> Self;

    /// Convert the registry value to an ID.
    fn as_id(&self) -> u32;
}

/// A trait for converting between keys and a registry values.
pub trait ConvertKey
where
    Self: Sized + Any,
{
    /// Convert the key to a registry value.
    ///
    /// # Errors
    /// If the key does not match any known value.
    fn try_from_key(key: &(impl AsRef<str> + ?Sized)) -> Result<Self, UnknownKeyError>;

    /// Convert the registry value to a key.
    fn as_key(&self) -> ResourceKey;
}

/// An error that occurs when an unknown key is encountered.
#[derive(Debug, Clone)]
pub struct UnknownKeyError {
    /// The name of the type.
    pub type_name: &'static str,
    /// The key that was used.
    pub key: ResourceKey,
}
