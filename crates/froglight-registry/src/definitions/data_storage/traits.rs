//! Traits for converting between registry values, protocol ids,
//! and resource keys.

use froglight_protocol::traits::Version;

/// A trait for initializing a registry with default values.
///
/// This trait must be implemented per [`Version`], as IDs and
/// default registry data changes between versions.
pub trait InitializeRegistry<V: Version>
where
    Self: Sized + Send + Sync,
{
    /// Initialize the registry with the default values.
    ///
    /// The order of the values is important, as it will be used
    /// to convert between IDs and registry values.
    #[must_use]
    fn initialize() -> Vec<Self>;
}
