//! TODO

use froglight_common::version::Version;

use crate::storage::BiomeStorage;
#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
use crate::storage::GlobalBiomeStorage;

/// A [`Version`]'s associated biome data.
pub trait BiomeVersion: Version {
    /// The [`GlobalBlockStorage`] for this [`Version`].
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    const BIOMES: &'static GlobalBiomeStorage;

    /// Get the [`GlobalBlockStorage`] for this [`Version`].
    #[inline]
    #[must_use]
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    fn biomes() -> &'static GlobalBiomeStorage { Self::BIOMES }

    /// Create a new [`BlockStorage`] for this [`Version`].
    ///
    /// # Warning
    ///
    /// This will create a new [`BlockStorage`] each time it is called!
    ///
    /// Unless you are in a `no_std` environment, you should probably be using
    /// [`BiomeVersion::biomes`] or the associated constant.
    fn new_biomes() -> BiomeStorage;
}
