//! TODO

use froglight_common::version::Version;
pub use froglight_registry_template::types::{LazyLock, RwLock};

use crate::storage::RegistryStorage;

/// A [`Version`] that has a [`RegistryStorage`].
pub trait RegistryVersion: Version {
    /// The [`RegistryStorage`] for this [`Version`].
    const REGISTRY: &'static LazyLock<RwLock<RegistryStorage>>;

    /// Get the [`RegistryStorage`] for this [`Version`].
    #[inline]
    #[must_use]
    fn registry() -> &'static RwLock<RegistryStorage> { Self::REGISTRY }

    /// Create a new [`RegistryStorage`] for this [`Version`].
    ///
    /// # Warning
    ///
    /// This will create a new [`RegistryStorage`] each time it is called!
    #[must_use]
    fn new_registry() -> RegistryStorage;
}
