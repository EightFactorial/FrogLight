//! TODO

use froglight_common::version::Version;

use crate::storage::BiomeStorage;
#[cfg(feature = "std")]
use crate::storage::GlobalBiomeStorage;

/// A [`Version`]'s associated biome data.
pub trait BiomeVersion: Version {
    /// The [`GlobalBiomeStorage`] for this [`Version`].
    #[cfg(feature = "std")]
    const BIOMES: &'static std::sync::LazyLock<GlobalBiomeStorage>;

    /// Get the [`GlobalBiomeStorage`] for this [`Version`].
    #[inline]
    #[must_use]
    #[cfg(feature = "std")]
    fn biomes() -> &'static GlobalBiomeStorage { Self::BIOMES }

    /// Create a new [`BiomeStorage`] for this [`Version`].
    ///
    /// # Warning
    ///
    /// This will create a new [`BiomeStorage`] each time it is called!
    ///
    /// Unless you are in a `no_std` environment, you should probably be using
    /// [`BiomeVersion::biomes`] or the associated constant.
    fn new_biomes() -> BiomeStorage;
}
