//! TODO

use froglight_common::version::Version;

use crate::storage::BlockStorage;
#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
use crate::storage::GlobalBlockStorage;

/// A [`Version`]'s associated block data.
pub trait BlockVersion: Version {
    /// The [`GlobalBlockStorage`] for this [`Version`].
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    const BLOCKS: &'static GlobalBlockStorage;

    /// Get the [`GlobalBlockStorage`] for this [`Version`].
    #[inline]
    #[must_use]
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    fn blocks() -> &'static GlobalBlockStorage { Self::BLOCKS }

    /// Create a new [`BlockStorage`] for this [`Version`].
    ///
    /// # Warning
    ///
    /// This will create a new [`BlockStorage`] each time it is called!
    ///
    /// Unless you are in a `no_std` environment, you should probably be using
    /// [`BlockVersion::blocks`] or the associated constant.
    fn new_blocks() -> BlockStorage;
}
