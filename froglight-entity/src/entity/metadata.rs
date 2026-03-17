#[cfg(feature = "bevy")]
use alloc::boxed::Box;
use core::{
    any::TypeId,
    fmt::{self, Debug},
};

#[cfg(feature = "bevy")]
use bevy_reflect::PartialReflect;
#[cfg(feature = "facet")]
use facet::Peek;
use froglight_common::prelude::Identifier;

use crate::{
    atomic::MaybeAtomicU32,
    entity::{EntityAabb, EntityDataSet, GlobalId, entity::EntityType},
    prelude::EntityVersion,
};

/// Metadata about an entity type.
pub struct EntityMetadata {
    /// The string identifier of the entity.
    identifier: Identifier<'static>,
    /// The [`GlobalId`] assigned to this entity.
    global_id: MaybeAtomicU32,
    /// The default [`EntityDataSet`] for this entity type.
    dataset: EntityDataSet<'static>,

    // The entity's bounding box.
    aabb: EntityAabb,

    #[cfg(feature = "bevy")]
    #[allow(clippy::type_complexity, reason = "Function pointers")]
    inspect_reflect: fn(&EntityDataSet, &mut dyn FnMut(Box<dyn PartialReflect>)),
    #[cfg(feature = "facet")]
    #[allow(clippy::type_complexity, reason = "Function pointers")]
    inspect_peek: fn(&EntityDataSet, &mut dyn FnMut(Peek<'_, '_>)),

    component_tys: &'static [TypeId],
    entity_ty: TypeId,
    version_ty: TypeId,
}

impl EntityMetadata {
    /// Create a new [`EntityMetadata`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `global_id` value is correct for the
    /// [`EntityStorage`](crate::storage::EntityStorage) it will be used in.
    #[must_use]
    #[allow(clippy::too_many_arguments, reason = "Yes")]
    #[allow(clippy::type_complexity, reason = "Function pointers")]
    pub const unsafe fn new<E: EntityType<V>, V: EntityVersion>(
        identifier: Identifier<'static>,
        size: [f32; 2],
        eye_height: f32,
        global_id: u32,
    ) -> Self {
        Self {
            identifier,
            global_id: MaybeAtomicU32::new(global_id),
            dataset: E::DATASET,

            aabb: EntityAabb::new(size[0] as f64, size[1] as f64, eye_height as f64),

            #[cfg(feature = "bevy")]
            inspect_reflect: E::inspect_reflect,
            #[cfg(feature = "facet")]
            inspect_peek: E::inspect_peek,

            component_tys: E::COMPONENTS,
            entity_ty: TypeId::of::<E>(),
            version_ty: TypeId::of::<V>(),
        }
    }

    /// Get the string identifier of this entity type.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Get the [`GlobalId`] of this entity type.
    #[must_use]
    pub fn global_id(&self) -> GlobalId { GlobalId::new(self.global_id.get()) }

    /// Get the default [`EntityDataSet`] for this entity type.
    #[must_use]
    pub fn default_data(&self) -> EntityDataSet<'static> { self.dataset.clone() }

    /// Get the entity's [`EntityAabb`].
    #[inline]
    #[must_use]
    pub const fn aabb(&self) -> &EntityAabb { &self.aabb }

    /// Returns `true` if this entity is of type `E`.
    #[must_use]
    pub fn is_entity<E: 'static>(&self) -> bool { self.entity_ty == TypeId::of::<E>() }

    /// Returns `true` if this version is of version `V`.
    #[must_use]
    pub fn is_version<V: 'static>(&self) -> bool { self.version_ty == TypeId::of::<V>() }

    /// Inspect the entity's component data using [`PartialReflect`].
    #[inline]
    #[cfg(feature = "bevy")]
    pub fn inspect_reflect(
        &self,
        dataset: &EntityDataSet,
        mut f: impl FnMut(Box<dyn PartialReflect>),
    ) {
        (self.inspect_reflect)(dataset, &mut f);
    }

    /// Inspect the entity's component data using [`Peek`].
    #[inline]
    #[cfg(feature = "facet")]
    pub fn inspect_peek(&self, dataset: &EntityDataSet, mut f: impl FnMut(Peek<'_, '_>)) {
        (self.inspect_peek)(dataset, &mut f);
    }

    /// Get the [`TypeId`]s of the entity's components.
    #[inline]
    #[must_use]
    pub const fn component_tys(&self) -> &'static [TypeId] { self.component_tys }

    /// Get the [`TypeId`] of the entity type.
    #[inline]
    #[must_use]
    pub const fn entity_ty(&self) -> TypeId { self.entity_ty }

    /// Get the [`TypeId`] of the version type.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version_ty }
}

impl Debug for EntityMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EntityMetadata").field(self.identifier()).finish_non_exhaustive()
    }
}

impl Eq for EntityMetadata {}
impl PartialEq for EntityMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.entity_ty == other.entity_ty && self.version_ty == other.version_ty
    }
}
