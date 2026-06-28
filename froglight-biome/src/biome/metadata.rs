use core::{any::TypeId, fmt::Debug};

use froglight_common::prelude::Identifier;
#[cfg(feature = "biome_data")]
use froglight_registry_template::types::LazyLock;

#[cfg(feature = "biome_data")]
use crate::biome::BiomeAttributeSet;
use crate::{biome::BiomeType, state::GlobalBiomeId, version::BiomeVersion};

/// Metadata about a biome type.
pub struct BiomeMetadata {
    /// The string identifier of the biome.
    identifier: Identifier<'static>,
    /// The [`GlobalBiomeId`] assigned to this biome.
    global_id: GlobalBiomeId,

    /// The foliage color of this biome.
    color_foliage: u32,
    /// The dry foliage color of this biome.
    color_foliage_dry: u32,
    /// The grass color of this biome.
    color_grass: u32,
    /// The water color of this biome.
    color_water: u32,
    /// Whether this biome has precipitation.
    precipitation: bool,
    /// The temperature of this biome.
    temperature: f32,
    /// The downfall of this biome.
    downfall: f32,

    /// The attributes of this biome.
    #[cfg(feature = "biome_data")]
    attributes: &'static LazyLock<BiomeAttributeSet>,

    /// The [`TypeId`] of the biome type.
    biome_ty: TypeId,
    /// The [`TypeId`] of the version type.
    version_ty: TypeId,
}

impl BiomeMetadata {
    /// Create a new [`BiomeMetadata`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `global_id` value is correct for the
    /// [`BiomeStorage`](crate::storage::BiomeStorage) it will be used in.
    #[must_use]
    #[expect(clippy::too_many_arguments, reason = "Yes")]
    pub const unsafe fn new<B: BiomeType<V>, V: BiomeVersion>(
        identifier: Identifier<'static>,
        global_id: GlobalBiomeId,
        foliage_color: u32,
        dry_foliage_color: u32,
        grass_color: u32,
        water_color: u32,
        precipitation: bool,
        temperature: f32,
        downfall: f32,
        #[cfg(feature = "biome_data")] attributes: &'static LazyLock<BiomeAttributeSet>,
    ) -> Self {
        Self {
            identifier,
            global_id,

            color_foliage: foliage_color,
            color_foliage_dry: dry_foliage_color,
            color_grass: grass_color,
            color_water: water_color,
            precipitation,
            temperature,
            downfall,

            #[cfg(feature = "biome_data")]
            attributes,

            biome_ty: TypeId::of::<B>(),
            version_ty: TypeId::of::<V>(),
        }
    }

    /// Get the string identifier of this biome.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Get the [`GlobalStateId`] of this biome.
    #[inline]
    #[must_use]
    pub const fn global_id(&self) -> GlobalBiomeId { self.global_id }

    /// Get the foliage color of this biome.
    #[inline]
    #[must_use]
    pub const fn foliage_color(&self) -> u32 { self.color_foliage }

    /// Get the dry foliage color of this biome.
    #[inline]
    #[must_use]
    pub const fn dry_foliage_color(&self) -> u32 { self.color_foliage_dry }

    /// Get the grass color of this biome.
    #[inline]
    #[must_use]
    pub const fn grass_color(&self) -> u32 { self.color_grass }

    /// Get the water color of this biome.
    #[inline]
    #[must_use]
    pub const fn water_color(&self) -> u32 { self.color_water }

    /// Returns `true` if this biome has precipitation.
    #[inline]
    #[must_use]
    pub const fn precipitation(&self) -> bool { self.precipitation }

    /// Get the temperature of this biome.
    #[inline]
    #[must_use]
    pub const fn temperature(&self) -> f32 { self.temperature }

    /// Get the downfall of this biome.
    #[inline]
    #[must_use]
    pub const fn downfall(&self) -> f32 { self.downfall }

    /// Get the attributes of this biome.
    #[inline]
    #[must_use]
    #[cfg(feature = "biome_data")]
    pub fn attributes(&self) -> &BiomeAttributeSet { self.attributes }

    /// Returns `true` if this biome is of type `B`.
    #[inline]
    #[must_use]
    pub fn is_biome<B: 'static>(&self) -> bool { self.biome_ty == TypeId::of::<B>() }

    /// Returns `true` if this biome is of version `V`.
    #[inline]
    #[must_use]
    pub fn is_version<V: 'static>(&self) -> bool { self.version_ty == TypeId::of::<V>() }

    /// Get the [`TypeId`] of the biome type.
    #[inline]
    #[must_use]
    pub const fn biome_ty(&self) -> TypeId { self.biome_ty }

    /// Get the [`TypeId`] of the version type.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version_ty }
}

impl Debug for BiomeMetadata {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BiomeMetadata").field(self.identifier()).finish_non_exhaustive()
    }
}
