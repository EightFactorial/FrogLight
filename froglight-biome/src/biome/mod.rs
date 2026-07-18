//! TODO

use core::{
    any::TypeId,
    cmp::Ordering,
    fmt::{self, Debug, Display},
};

use froglight_common::prelude::Identifier;

use crate::{state::GlobalBiomeId, storage::BiomeStorage, version::BiomeVersion};

#[cfg(feature = "biome_data")]
mod attribute;
#[cfg(feature = "biome_data")]
pub use attribute::{AttributeType, BiomeAttributeSet};

mod metadata;
pub use metadata::BiomeMetadata;

/// A biome in the world.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Biome {
    metadata: &'static BiomeMetadata,
}

impl Biome {
    /// Create a new [`Biome`] of the given type.
    #[inline]
    #[must_use]
    pub const fn new<B: BiomeType<V>, V: BiomeVersion>() -> Self { Self::new_from(B::METADATA) }

    /// Create a new [`Biome`] from the given metadata.
    #[inline]
    #[must_use]
    pub const fn new_from(metadata: &'static BiomeMetadata) -> Self { Biome { metadata } }

    /// Get the string identifier of this biome.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> Identifier<'static> { self.metadata.identifier().reborrow() }

    /// Get the [`BiomeMetadata`] of this biome.
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &'static BiomeMetadata { self.metadata }

    /// Get the [`GlobalStateId`] of this biome.
    #[inline]
    #[must_use]
    pub fn global_id(&self) -> GlobalBiomeId { self.metadata.global_id() }

    /// Attempt to migrate the biome to another [`BiomeVersion`].
    ///
    /// Returns `None` if there is no matching [`BiomeType`].
    #[inline]
    #[must_use]
    pub fn using_version<V: BiomeVersion>(self) -> Option<Biome> {
        self.using_version_storage(V::biomes())
    }

    /// Attempt to migrate the biome to another [`BiomeVersion`].
    ///
    /// Equivalent to [`Biome::using_version`] but without the generic
    /// parameter.
    ///
    /// Returns `None` if there is no matching [`BiomeType`].
    #[must_use]
    pub fn using_version_storage(self, biomes: &BiomeStorage) -> Option<Biome> {
        // If the `Version` is the same, do nothing.
        if self.version_ty() == biomes.version_ty() {
            return Some(self);
        }

        // Try the biome with a matching identifier and type.
        if let Some(biome) = biomes.get_biome_by_identifier(&self.identifier())
            && self.biome_ty() == biome.biome_ty()
        {
            return Some(biome);
        }

        // Otherwise, iterate over all biomes for a matching type.
        biomes.metadata().iter().find_map(|(ident, meta)| {
            if self.biome_ty() == meta.biome_ty() {
                biomes.get_biome_by_identifier(ident)
            } else {
                None
            }
        })
    }
}

froglight_registry_template::implement_wrapper! {
    impl Biome {
        [ () => metadata ]

        /// Get the grass color of this biome.
        #[inline]
        #[must_use]
        pub fn grass_color(&self) -> u32;

        /// Get the foliage color of this biome.
        #[inline]
        #[must_use]
        pub fn foliage_color(&self) -> u32;

        /// Get the dry foliage color of this biome.
        #[inline]
        #[must_use]
        pub fn dry_foliage_color(&self) -> u32;

        /// Get the water color of this biome.
        #[inline]
        #[must_use]
        pub fn water_color(&self) -> u32;

        /// Get the water fog color of this biome.
        #[inline]
        #[must_use]
        pub fn precipitation(&self) -> bool;

        /// Get the temperature of this biome.
        #[inline]
        #[must_use]
        pub fn temperature(&self) -> f32;

        /// Get the downfall of this biome.
        #[inline]
        #[must_use]
        pub fn downfall(&self) -> f32;

        /// Returns `true` if this biome is of type `B`.
        #[inline]
        #[must_use]
        pub fn is_biome<B: 'static>(&self) -> bool ;

        /// Returns `true` if this biome is of version `V`.
        #[inline]
        #[must_use]
        pub fn is_version<V: 'static>(&self) -> bool;

        /// Get the [`TypeId`] of the biome type.
        #[inline]
        #[must_use]
        pub fn biome_ty(&self) -> TypeId;

        /// Get the [`TypeId`] of the version type.
        #[inline]
        #[must_use]
        pub fn version_ty(&self) -> TypeId;
    }
}

// -------------------------------------------------------------------------------------------------

impl Eq for Biome {}
impl PartialEq for Biome {
    fn eq(&self, other: &Self) -> bool { self.metadata.global_id() == other.metadata.global_id() }
}

impl PartialOrd for Biome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.version_ty() == other.version_ty() {
            Some(self.metadata.global_id().cmp(&other.metadata.global_id()))
        } else {
            None
        }
    }
}

impl Display for Biome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { Display::fmt(&self.identifier(), f) }
}

impl Debug for Biome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Biome")
            .field(&self.identifier())
            .field(&self.global_id().into_inner())
            .finish_non_exhaustive()
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all biome types.
pub trait BiomeType<V: BiomeVersion>: 'static {
    /// The [`BiomeMetadata`] for this biome type.
    const METADATA: &'static BiomeMetadata;
}
