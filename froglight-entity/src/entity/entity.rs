use froglight_common::prelude::Identifier;

use crate::{
    entity::{EntityDataSet, GlobalId, metadata::EntityMetadata},
    prelude::EntityVersion,
};

/// A bundle of data and metadata for an entity.
#[derive(Clone)]
pub struct EntityBundle {
    #[expect(dead_code, reason = "WIP")]
    dataset: EntityDataSet<'static>,
    reference: &'static EntityMetadata,
}

impl EntityBundle {
    /// Create a new [`Biome`] of the given type.
    #[inline]
    #[must_use]
    pub fn new<E: EntityType<V>, V: EntityVersion>() -> Self { Self::new_from(E::METADATA) }

    /// Create a new [`Biome`] from the given metadata.
    #[inline]
    #[must_use]
    pub fn new_from(metadata: &'static EntityMetadata) -> Self {
        EntityBundle { dataset: metadata.default_data(), reference: metadata }
    }

    /// Create a new [`EntityBundle`] from the given [`EntityDataSet`] and
    /// [`EntityMetadata`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given `dataset` is valid for the
    /// metadata.
    #[must_use]
    pub const unsafe fn new_unchecked(
        dataset: EntityDataSet<'static>,
        metadata: &'static EntityMetadata,
    ) -> Self {
        Self { dataset, reference: metadata }
    }

    /// Get the string identifier of this biome.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { self.reference.identifier() }

    /// Get the [`BiomeMetadata`] of this biome.
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &'static EntityMetadata { self.reference }

    /// Get the [`GlobalId`] of this biome.
    #[inline]
    #[must_use]
    pub fn global_id(&self) -> GlobalId { self.reference.global_id() }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all entity types.
pub trait EntityType<V: EntityVersion>: 'static {
    /// The [`EntityMetadata`] for this entity type.
    const METADATA: &'static EntityMetadata;
}
