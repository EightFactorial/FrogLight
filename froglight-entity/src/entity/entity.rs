#[cfg(feature = "bevy")]
use alloc::boxed::Box;
use core::any::TypeId;

#[cfg(feature = "bevy")]
use bevy_reflect::PartialReflect;
#[cfg(feature = "facet")]
use facet::Peek;
use froglight_common::prelude::Identifier;

use crate::{
    entity::{EntityDataSet, GlobalId, metadata::EntityMetadata},
    prelude::EntityVersion,
};

/// A bundle of data and metadata for an entity.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityBundle {
    dataset: EntityDataSet<'static>,
    reference: &'static EntityMetadata,
}

impl EntityBundle {
    /// Create a new [`EntityBundle`] of the given type.
    #[inline]
    #[must_use]
    pub fn new<E: EntityType<V>, V: EntityVersion>() -> Self { Self::new_from(E::METADATA) }

    /// Create a new [`EntityBundle`] from the given metadata.
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

    /// Get a reference to the [`EntityDataSet`] of this entity.
    #[inline]
    #[must_use]
    pub const fn dataset(&self) -> &EntityDataSet<'static> { &self.dataset }

    /// Get a mutable reference to the [`EntityDataSet`] of this entity.
    ///
    /// # Safety
    ///
    /// The caller must ensure the dataset is still valid for this entity after
    /// mutation.
    #[inline]
    #[must_use]
    pub const unsafe fn dataset_mut(&mut self) -> &mut EntityDataSet<'static> { &mut self.dataset }

    /// Get the string identifier of this entity.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { self.reference.identifier() }

    /// Get the [`EntityMetadata`] of this entity.
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &'static EntityMetadata { self.reference }

    /// Get the [`GlobalId`] of this entity type.
    #[inline]
    #[must_use]
    pub fn global_id(&self) -> GlobalId { self.reference.global_id() }

    /// Inspect this entity's data using the given function.
    ///
    /// Requires the `bevy` feature to be enabled.
    #[cfg(feature = "bevy")]
    pub fn inspect_reflect(&self, f: impl FnMut(Box<dyn PartialReflect>)) {
        self.reference.inspect_reflect(&self.dataset, f);
    }

    /// Inspect this entity's data using the given function.
    ///
    /// Requires the `facet` feature to be enabled.
    #[cfg(feature = "facet")]
    pub fn inspect_peek(&self, f: impl FnMut(Peek<'_, '_>)) {
        self.reference.inspect_peek(&self.dataset, f);
    }

    /// Returns `true` if this entity is of type `E`.
    #[inline]
    #[must_use]
    pub fn is_entity<E: 'static>(&self) -> bool { self.reference.is_entity::<E>() }

    /// Returns `true` if this entity is of version `V`.
    #[inline]
    #[must_use]
    pub fn is_version<V: 'static>(&self) -> bool { self.reference.is_version::<V>() }

    /// Get the [`TypeId`] of the entity type.
    #[inline]
    #[must_use]
    pub const fn entity_ty(&self) -> TypeId { self.reference.entity_ty() }

    /// Get the [`TypeId`] of the version type.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.reference.version_ty() }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all entity types.
pub trait EntityType<V: EntityVersion>: 'static {
    /// The [`EntityMetadata`] for this entity type.
    const METADATA: &'static EntityMetadata;
}
