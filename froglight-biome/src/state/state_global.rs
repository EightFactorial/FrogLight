//! TODO

#[cfg(feature = "facet")]
use froglight_facet as mc;

use crate::{biome::Biome, prelude::BiomeVersion};

/// A unique identifier for a biome,
/// relative to all other biomes in the same version.
///
/// This only guarantees uniqueness if both biomes are, for example,
/// from [`V26_1`](froglight_common::prelude::V26_1).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(transparent, mc::variable_inner))]
pub struct GlobalBiomeId(u32);

impl GlobalBiomeId {
    /// Create a new [`GlobalBiomeId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u32) -> Self { GlobalBiomeId(id) }

    /// Try to convert this [`GlobalBiomeId`] into a [`Biome`].
    ///
    /// See [`BlockStorage::get_biome_by_id`](crate::storage::BiomeStorage::get_biome_by_id).
    #[inline]
    #[must_use]
    pub fn try_into_block<V: BiomeVersion>(self) -> Option<Biome> {
        V::biomes().get_biome_by_id(self)
    }

    /// Get the inner [`u32`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u32 { self.0 }
}

impl<T: Into<u32>> From<T> for GlobalBiomeId {
    #[inline]
    fn from(value: T) -> Self { GlobalBiomeId(value.into()) }
}
impl From<Biome> for GlobalBiomeId {
    #[inline]
    fn from(value: Biome) -> Self { value.global_id() }
}

impl<T: PartialEq<u32>> PartialEq<T> for GlobalBiomeId {
    #[inline]
    fn eq(&self, other: &T) -> bool { other.eq(&self.0) }
}
