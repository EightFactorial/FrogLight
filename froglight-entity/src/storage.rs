//! TODO

use alloc::vec::Vec;
use core::any::TypeId;

use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use indexmap::IndexMap;

use crate::{
    entity::{EntityBundle, EntityMetadata, GlobalEntityId},
    version::EntityVersion,
};

/// A container for entity data storage.
#[derive(Debug, Clone)]
pub struct EntityStorage {
    version: TypeId,
    metadata: IndexMap<Identifier<'static>, &'static EntityMetadata, RandomState>,
}

impl EntityStorage {
    /// Build a new [`EntityStorage`] for the given [`EntityVersion`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that all provided entity metadata has the correct
    /// global ids for this collection.
    #[must_use]
    pub unsafe fn build<V: EntityVersion>(metadata: Vec<&'static EntityMetadata>) -> Self {
        let mut identifiers =
            IndexMap::with_capacity_and_hasher(metadata.len(), RandomState::default());

        for meta in metadata {
            identifiers.entry(meta.identifier().reborrow()).insert_entry(meta);
        }

        Self { version: TypeId::of::<V>(), metadata: identifiers }
    }

    /// Get the default [`EntityBundle`] for a given [`GlobalEntityId`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry and world.
    #[must_use]
    pub fn get_entity_by_id(&self, id: GlobalEntityId) -> Option<EntityBundle> {
        self.metadata
            .get_index(id.into_inner() as usize)
            .map(|(_, meta)| EntityBundle::new_from(meta))
    }

    /// Get the [`Entity`] for a given [`Identifier`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry.
    #[must_use]
    pub fn get_entity_by_identifier(&self, identifier: &Identifier<'_>) -> Option<EntityBundle> {
        self.metadata.get(identifier).map(|&meta| EntityBundle::new_from(meta))
    }

    /// Get the [`TypeId`] of the [`Version`] this storage is for.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version }

    /// Get the [`IndexMap`] metadata of this [`EntityStorage`].
    #[inline]
    #[must_use]
    pub const fn metadata(
        &self,
    ) -> &IndexMap<Identifier<'static>, &'static EntityMetadata, RandomState> {
        &self.metadata
    }
}

// -------------------------------------------------------------------------------------------------

/// A macro helper for implementing
/// [`EntityVersion`](crate::version::EntityVersion) for a given
/// [`Version`](froglight_common::version::Version).
#[macro_export]
macro_rules! implement_entities {
    ($version:ty => { $($tt:tt)* }, read: $read:block, write: $write:block) => {
        impl $crate::version::EntityVersion for $version {
            const ENTITY: &'static $crate::version::LazyLock<$crate::version::AtomicArc<$crate::storage::EntityStorage>> = {
                static STATIC: $crate::version::LazyLock<$crate::version::AtomicArc<$crate::storage::EntityStorage>> = $crate::version::LazyLock::new(|| {
                    $crate::version::AtomicArc::from(<$version as $crate::version::EntityVersion>::new_entity())
                });
                &STATIC
            };

            fn new_entity() -> $crate::storage::EntityStorage {
               $($tt)*
            }

            #[cfg(feature = "facet")]
            const DATATYPE_DESERIALIZE: fn(
                protocol: u32,
                &mut froglight_facet::facet::template::Reader,
            ) -> Result<$crate::generated::datatype::EntityDataType, froglight_facet::facet::template::ReaderError>  = $read;

            #[cfg(feature = "facet")]
            const DATATYPE_SERIALIZE: fn(
                &$crate::generated::datatype::EntityDataType,
                protocol: u32,
                &mut froglight_facet::facet::template::Writer,
            ) -> Result<(), froglight_facet::facet::template::WriterError> = $write;
        }
    };
}
