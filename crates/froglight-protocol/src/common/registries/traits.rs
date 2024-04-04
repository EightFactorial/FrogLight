use crate::{common::ResourceKey, traits::Version};

/// A trait for converting an ID to a type.
pub trait FromId<V: Version> {
    /// Converts an ID to a type.
    fn from_id(id: u32);
}

/// A trait for converting a key to a type.
pub trait TryFromKey: Sized {
    /// Converts a key to a type.
    ///
    /// # Errors
    /// Returns an [`UnknownKeyError`] if the key is not recognized.
    fn try_from_key(key: &impl AsRef<str>) -> Result<Self, UnknownKeyError>;
}

/// An error that occurs when an unknown key is encountered.
#[derive(Debug, Clone)]
pub struct UnknownKeyError {
    /// The name of the type.
    pub type_name: &'static str,
    /// The key that was used.
    pub key: ResourceKey,
}
