//! Traits for converting between registry values, protocol ids,
//! and resource keys.

use froglight_protocol::{common::ResourceKey, traits::Version};
use hashbrown::HashMap;

/// A trait for initializing a registry with default values.
///
/// This trait must be implemented per [`Version`], as IDs and
/// default registry data changes between versions.
pub trait InitializeRegistry<V: Version>
where
    Self: Sized + Send + Sync,
{
    /// Initialize the ID storage with the default values.
    ///
    /// The order of the values is important, as it will be used
    /// to convert between IDs and registry values.
    #[must_use]
    fn initialize_ids() -> Vec<Self>;

    /// Initialize the data storage with the default values.
    ///
    /// This is used to store data for the registry values.
    #[must_use]
    fn initialize_storage() -> HashMap<ResourceKey, serde_json::Value> { HashMap::new() }
}
