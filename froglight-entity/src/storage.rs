//! TODO

use core::any::TypeId;

use froglight_common::{crates::indexmap::map::Entry, prelude::*, types::IndexMap};

use crate::{
    entity::{EntityBundle, EntityMetadata, GlobalEntityId},
    version::EntityVersion,
};

/// A container for entity data storage.
#[derive(Debug, Clone)]
pub struct EntityStorage {
    version: TypeId,
    metadata: IndexMap<&'static Ident, &'static EntityMetadata>,
}

impl EntityStorage {
    /// Build a new [`EntityStorage`] for the given [`EntityVersion`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that all provided [`EntityMetadata`] is valid for
    /// this [`Version`], and has a matching entry per-[`GlobalEntityId`].
    ///
    /// # Panics
    ///
    /// Panics in `debug` builds if the provided [`EntityMetadata`] is invalid.
    #[must_use]
    pub unsafe fn build<V: EntityVersion>(metadata: &[&'static EntityMetadata]) -> Self {
        let mut identifiers = IndexMap::with_capacity_and_hasher(metadata.len(), <_>::default());

        for (_index, meta) in metadata.iter().enumerate() {
            #[cfg(debug_assertions)]
            #[expect(clippy::used_underscore_binding, reason = "Debug assertions")]
            Self::assert_entity::<V>(_index, meta);

            match identifiers.entry(meta.identifier()) {
                Entry::Vacant(entry) => _ = entry.insert(*meta),
                Entry::Occupied(..) => {
                    core::hint::cold_path();

                    #[cfg(debug_assertions)]
                    panic!("EntityMetadata has duplicate identifier: {:?}", meta.identifier());
                }
            }
        }

        Self { version: TypeId::of::<V>(), metadata: identifiers }
    }

    #[cfg(debug_assertions)]
    fn assert_entity<V: EntityVersion>(index: usize, meta: &EntityMetadata) {
        debug_assert_eq!(
            index,
            meta.global_id().into_usize(),
            "GlobalEntityId `{index}` does not match the expected value for {:?}: `{}`",
            meta.identifier(),
            meta.global_id().into_inner(),
        );

        debug_assert!(
            meta.is_version::<V>(),
            "EntityMetadata Version mismatch for {:?}: expected {:?}",
            meta.identifier(),
            core::any::type_name::<V>()
        );
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

    /// Get the [`EntityBundle`] for a given [`Identifier`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry.
    #[must_use]
    pub fn get_entity_by_identifier(&self, identifier: &Ident) -> Option<EntityBundle> {
        self.metadata.get(identifier).map(|&meta| EntityBundle::new_from(meta))
    }

    /// Get the [`TypeId`] of the [`Version`] this storage is for.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version }

    /// Get the [`IndexMap`] metadata of this [`EntityStorage`].
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &IndexMap<&'static Ident, &'static EntityMetadata> {
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
            const ENTITY: &'static $crate::version::OnceLock<$crate::storage::EntityStorage> = {
                static STATIC: $crate::version::OnceLock<$crate::storage::EntityStorage> = $crate::version::OnceLock::new();
                &STATIC
            };

            fn new_entity() -> $crate::storage::EntityStorage {
               $($tt)*
            }

            #[cfg(feature = "facet")]
            const DATATYPE_DESERIALIZE: fn(
                &mut froglight_facet::facet::prelude::Reader,
            ) -> Result<$crate::generated::datatype::EntityDataType, froglight_facet::facet::prelude::ReaderError>  = $read;

            #[cfg(feature = "facet")]
            const DATATYPE_SERIALIZE: fn(
                &$crate::generated::datatype::EntityDataType,
                &mut froglight_facet::facet::prelude::Writer,
            ) -> Result<(), froglight_facet::facet::prelude::WriterError> = $write;
        }
    };
}
