use core::{
    any::TypeId,
    fmt::{self, Debug},
};

use froglight_common::prelude::Identifier;

use crate::{
    atomic::MaybeAtomicU32,
    entity::{EntityDataSet, GlobalId, entity::EntityType},
    prelude::EntityVersion,
};

/// Metadata about an entity type.
pub struct EntityMetadata {
    /// The string identifier of the entity.
    identifier: Identifier<'static>,
    /// The [`GlobalId`] assigned to this entity.
    global_id: MaybeAtomicU32,

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
    pub const unsafe fn new<E: EntityType<V>, V: EntityVersion>(
        identifier: Identifier<'static>,
        global_id: u32,
    ) -> Self {
        Self {
            identifier,
            global_id: MaybeAtomicU32::new(global_id),

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
    pub fn default_data(&self) -> EntityDataSet<'static> { todo!() }

    /// Returns `true` if this entity is of type `E`.
    #[must_use]
    pub fn is_entity<E: 'static>(&self) -> bool { self.entity_ty == TypeId::of::<E>() }

    /// Returns `true` if this version is of version `V`.
    #[must_use]
    pub fn is_version<V: 'static>(&self) -> bool { self.version_ty == TypeId::of::<V>() }

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
