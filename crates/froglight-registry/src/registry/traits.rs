use froglight_common::version::Version;

use super::RegistryStorage;

/// A trait for inserting registries into a [`RegistryStorage`].
pub trait RegistryTrait<V: Version> {
    /// Register all known registries and their values
    /// with the given [`RegistryStorage`].
    fn register(storage: &mut RegistryStorage<V>);
}
