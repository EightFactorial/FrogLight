//! TODO

use froglight_common::version::Version;

#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
use crate::storage::GlobalItemStorage;
use crate::storage::ItemStorage;

/// A [`Version`]'s associated item data.
pub trait ItemVersion: Version {
    /// The [`GlobalItemStorage`] for this [`Version`].
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    const ITEMS: &'static GlobalItemStorage;

    /// Get the [`GlobalItemStorage`] for this [`Version`].
    #[inline]
    #[must_use]
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    fn items() -> &'static GlobalItemStorage { Self::ITEMS }

    /// Create a new [`ItemStorage`] for this [`Version`].
    ///
    /// # Warning
    ///
    /// This will create a new [`ItemStorage`] each time it is called!
    ///
    /// Unless you are in a `no_std` environment, you should probably be using
    /// [`ItemVersion::items`] or the associated constant.
    fn new_items() -> ItemStorage;
}
