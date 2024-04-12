//! Traits for converting between registry values, protocol ids,
//! and resource keys.

use compact_str::CompactString;
use froglight_protocol::{common::ResourceKey, traits::Version};

/// A trait for converting between keys and a registry values.
pub trait ConvertKey
where
    Self: Sized + Send + Sync,
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
    pub key: CompactString,
}

impl UnknownKeyError {
    /// Create a new unknown key error.
    #[must_use]
    pub fn new<T: ConvertKey>(key: &str) -> Self {
        Self { type_name: std::any::type_name::<T>(), key: CompactString::new(key) }
    }
}

/// A trait for initializing a registry with default values.
///
/// This trait must be implemented per [`Version`], as IDs can
/// change between versions.
pub trait InitializeRegistry<V: Version>
where
    Self: Sized + Send + Sync,
{
    /// Initialize the registry with default values.
    ///
    /// The order of the values is important, as it will be used
    /// to convert between IDs and values.
    fn initialize() -> Vec<Self>;
}

/// A registry type that can only be modified by the application.
///
/// Upon leaving a server, the registry will overwrite the
/// [`runtime values`](RuntimeRegistry).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefaultRegistry;

pub(super) mod sealed {
    /// A type of registry.
    pub trait RegistryType {}
}
use sealed::RegistryType;

impl RegistryType for DefaultRegistry {}

/// A registry type that can be modified by connected servers.
///
/// Upon leaving a server, the registry will be reset to the
/// [`default values`](DefaultRegistry).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuntimeRegistry;

impl RegistryType for RuntimeRegistry {}
