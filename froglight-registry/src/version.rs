//! TODO

use froglight_common::version::Version;

#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
use crate::storage::GlobalRegistrySetStorage;
use crate::storage::RegistrySetStorage;

/// A [`Version`]'s associated registry data.
pub trait RegistryVersion: Version {
    /// The [`GlobalRegistryStorage`] for this [`Version`].
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    const REGISTRIES: &'static GlobalRegistrySetStorage;

    /// Get the [`GlobalRegistryStorage`] for this [`Version`].
    #[inline]
    #[must_use]
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    fn registries() -> &'static GlobalRegistrySetStorage { Self::REGISTRIES }

    /// Create a new [`RegistrySetStorage`] for this [`Version`].
    ///
    /// # Warning
    ///
    /// This will create a new [`RegistrySetStorage`] each time it is called!
    ///
    /// Unless you are in a `no_std` environment, you should probably be using
    /// [`RegistryVersion::registries`] or the associated constant.
    fn new_registries() -> RegistrySetStorage;
}
