use core::{any::TypeId, fmt::Debug};
#[cfg(feature = "std")]
use std::sync::LazyLock;

use froglight_common::prelude::Identifier;
#[cfg(all(feature = "once_cell", not(feature = "std")))]
use once_cell::sync::OnceCell as LazyLock;

use crate::{
    atomic::{MaybeAtomicBool, MaybeAtomicF32, MaybeAtomicU32},
    biome::{BiomeAttributeSet, BiomeType, GlobalId},
    version::BiomeVersion,
};

/// Metadata about a biome type.
pub struct BiomeMetadata {
    /// The string identifier of the biome.
    identifier: Identifier<'static>,
    /// The [`GlobalId`] assigned to this biome.
    global_id: MaybeAtomicU32,

    /// The foliage color of this biome.
    color_foliage: MaybeAtomicU32,
    /// The dry foliage color of this biome.
    color_foliage_dry: MaybeAtomicU32,
    /// The grass color of this biome.
    color_grass: MaybeAtomicU32,
    /// The water color of this biome.
    color_water: MaybeAtomicU32,
    /// Whether this biome has precipitation.
    precipitation: MaybeAtomicBool,
    /// The temperature of this biome.
    temperature: MaybeAtomicF32,
    /// The downfall of this biome.
    downfall: MaybeAtomicF32,

    /// The attributes of this biome.
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
        global_id: u32,
        foliage_color: u32,
        dry_foliage_color: u32,
        grass_color: u32,
        water_color: u32,
        precipitation: bool,
        temperature: f32,
        downfall: f32,
        attributes: &'static LazyLock<BiomeAttributeSet>,
    ) -> Self {
        Self {
            identifier,
            global_id: MaybeAtomicU32::new(global_id),

            color_foliage: MaybeAtomicU32::new(foliage_color),
            color_foliage_dry: MaybeAtomicU32::new(dry_foliage_color),
            color_grass: MaybeAtomicU32::new(grass_color),
            color_water: MaybeAtomicU32::new(water_color),
            precipitation: MaybeAtomicBool::new(precipitation),
            temperature: MaybeAtomicF32::new(temperature),
            downfall: MaybeAtomicF32::new(downfall),

            attributes,

            biome_ty: TypeId::of::<B>(),
            version_ty: TypeId::of::<V>(),
        }
    }

    /// Get the string identifier of this biome.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Get the [`GlobalId`] of this biome.
    #[must_use]
    pub fn global_id(&self) -> GlobalId { GlobalId::new(self.global_id.get()) }

    /// Set the base [`GlobalId`] of this biome.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the new id matches the indices in the
    /// [`BiomeStorage`](crate::storage::BiomeStorage) it is used in.
    #[cfg(feature = "atomic")]
    pub unsafe fn set_global_id(&self, id: GlobalId) { self.global_id.set_atomic(id.into_inner()); }

    /// Get the foliage color of this biome.
    #[must_use]
    pub fn foliage_color(&self) -> u32 { self.color_foliage.get() }

    /// Set the foliage color of this biome.
    #[cfg(feature = "atomic")]
    pub fn set_foliage_color(&self, color: u32) { self.color_foliage.set_atomic(color); }

    /// Get the dry foliage color of this biome.
    #[must_use]
    pub fn dry_foliage_color(&self) -> u32 { self.color_foliage_dry.get() }

    /// Set the dry foliage color of this biome.
    #[cfg(feature = "atomic")]
    pub fn set_dry_foliage_color(&self, color: u32) { self.color_foliage_dry.set_atomic(color); }

    /// Get the grass color of this biome.
    #[must_use]
    pub fn grass_color(&self) -> u32 { self.color_grass.get() }

    /// Set the grass color of this biome.
    #[cfg(feature = "atomic")]
    pub fn set_grass_color(&self, color: u32) { self.color_grass.set_atomic(color); }

    /// Get the water color of this biome.
    #[must_use]
    pub fn water_color(&self) -> u32 { self.color_water.get() }

    /// Set the water color of this biome.
    #[cfg(feature = "atomic")]
    pub fn set_water_color(&self, color: u32) { self.color_water.set_atomic(color); }

    /// Returns `true` if this biome has precipitation.
    #[must_use]
    pub fn precipitation(&self) -> bool { self.precipitation.get() }

    /// Set whether this biome has precipitation.
    #[cfg(feature = "atomic")]
    pub fn set_precipitation(&self, precipitation: bool) {
        self.precipitation.set_atomic(precipitation);
    }

    /// Get the temperature of this biome.
    #[must_use]
    pub fn temperature(&self) -> f32 { self.temperature.get() }

    /// Set the temperature of this biome.
    #[cfg(feature = "atomic")]
    pub fn set_temperature(&self, temperature: f32) { self.temperature.set_atomic(temperature); }

    /// Get the downfall of this biome.
    #[must_use]
    pub fn downfall(&self) -> f32 { self.downfall.get() }

    /// Set the downfall of this biome.
    #[cfg(feature = "atomic")]
    pub fn set_downfall(&self, downfall: f32) { self.downfall.set_atomic(downfall); }

    /// Get the attributes of this biome.
    #[must_use]
    pub fn attributes(&self) -> &BiomeAttributeSet { self.attributes }

    /// Returns `true` if this biome is of type `B`.
    #[must_use]
    pub fn is_biome<B: 'static>(&self) -> bool { self.biome_ty == TypeId::of::<B>() }

    /// Returns `true` if this biome is of version `V`.
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
