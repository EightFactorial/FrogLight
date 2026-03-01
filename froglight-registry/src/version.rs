//! TODO

use froglight_common::version::Version;

#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
use crate::storage::GlobalRegistrySetStorage;
use crate::storage::RegistrySetStorage;

/// A [`Version`]'s associated registry data.
pub trait RegistryVersion: Version {
    /// The [`GlobalRegistrySetStorage`] for this [`Version`].
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    const REGISTRY: &'static GlobalRegistrySetStorage;

    /// Get the [`GlobalRegistrySetStorage`] for this [`Version`].
    #[inline]
    #[must_use]
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    fn registry() -> &'static GlobalRegistrySetStorage { Self::REGISTRY }

    /// Create a new [`RegistrySetStorage`] for this [`Version`].
    ///
    /// # Warning
    ///
    /// This will create a new [`RegistrySetStorage`] each time it is called!
    ///
    /// Unless you are in a `no_std` environment, you should probably be using
    /// [`RegistryVersion::registries`] or the associated constant.
    fn new_registry() -> RegistrySetStorage;
}
