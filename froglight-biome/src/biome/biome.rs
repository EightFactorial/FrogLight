use core::{
    any::TypeId,
    cmp::Ordering,
    fmt::{self, Debug, Display},
};

use froglight_common::prelude::Identifier;

use crate::{
    biome::{BiomeMetadata, GlobalId},
    version::BiomeVersion,
};

/// A biome in the world.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Biome {
    reference: &'static BiomeMetadata,
}

impl Biome {
    /// Create a new [`Biome`] of the given type.
    #[inline]
    #[must_use]
    pub const fn new<B: BiomeType<V>, V: BiomeVersion>() -> Self { Self::new_from(B::METADATA) }

    /// Create a new [`Biome`] from the given metadata.
    #[inline]
    #[must_use]
    pub const fn new_from(metadata: &'static BiomeMetadata) -> Self {
        Biome { reference: metadata }
    }

    /// Get the string identifier of this biome.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { self.reference.identifier() }

    /// Get the [`BiomeMetadata`] of this biome.
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &'static BiomeMetadata { self.reference }

    /// Get the [`GlobalId`] of this biome.
    #[inline]
    #[must_use]
    pub fn global_id(&self) -> GlobalId { self.reference.global_id() }

    /// Get the grass color of this biome.
    #[inline]
    #[must_use]
    pub fn grass_color(&self) -> u32 { self.reference.grass_color() }

    /// Get the foliage color of this biome.
    #[inline]
    #[must_use]
    pub fn foliage_color(&self) -> u32 { self.reference.foliage_color() }

    /// Get the water color of this biome.
    #[inline]
    #[must_use]
    pub fn water_color(&self) -> u32 { self.reference.water_color() }

    /// Get the water fog color of this biome.
    #[inline]
    #[must_use]
    pub fn precipitation(&self) -> bool { self.reference.precipitation() }

    /// Get the temperature of this biome.
    #[inline]
    #[must_use]
    pub fn temperature(&self) -> f32 { self.reference.temperature() }

    /// Get the downfall of this biome.
    #[inline]
    #[must_use]
    pub fn downfall(&self) -> f32 { self.reference.downfall() }

    /// Returns `true` if this biome is of type `B`.
    #[inline]
    #[must_use]
    pub fn is_block<B: 'static>(&self) -> bool { self.reference.is_biome::<B>() }

    /// Returns `true` if this biome is of version `V`.
    #[inline]
    #[must_use]
    pub fn is_version<V: 'static>(&self) -> bool { self.reference.is_version::<V>() }

    /// Get the [`TypeId`] of the biome type.
    #[inline]
    #[must_use]
    pub const fn block_ty(&self) -> TypeId { self.reference.block_ty() }

    /// Get the [`TypeId`] of the version type.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.reference.version_ty() }
}

impl Eq for Biome {}
impl PartialEq for Biome {
    fn eq(&self, other: &Self) -> bool { self.reference.global_id() == other.reference.global_id() }
}

impl PartialOrd for Biome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.version_ty() == other.version_ty() {
            Some(self.reference.global_id().cmp(&other.reference.global_id()))
        } else {
            None
        }
    }
}

impl Display for Biome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { Display::fmt(self.identifier(), f) }
}

impl Debug for Biome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Biome")
            .field(self.reference.identifier())
            .field(&self.global_id().into_inner())
            .finish_non_exhaustive()
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all biome types.
pub trait BiomeType<V: BiomeVersion>: 'static {
    /// The [`BiomeMetadata`] for this block type.
    const METADATA: &'static BiomeMetadata;
}
