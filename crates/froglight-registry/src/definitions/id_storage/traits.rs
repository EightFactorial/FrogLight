//! Traits for converting between registry values, protocol ids,
//! and resource keys.

use froglight_protocol::traits::Version;

/// A trait for initializing a registry with default values.
///
/// This trait must be implemented per [`Version`], as IDs can
/// change between versions.
pub trait InitializeIdRegistry<V: Version>
where
    Self: Sized + Send + Sync,
{
    /// Initialize the registry with default values.
    ///
    /// The order of the values is important, as it will be used
    /// to convert between IDs and values.
    fn initialize() -> Vec<Self>;
}
