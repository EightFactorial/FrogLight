//! TODO

use core::any::TypeId;

use froglight_common::{crates::indexmap::map::Entry, prelude::*, types::IndexMap};

use crate::{
    block::{Block, BlockMetadata},
    prelude::BlockVersion,
    state::{GlobalBlockId, GlobalStateId, RelativeStateId},
};

/// A container for block data storage.
#[derive(Debug, Clone)]
pub struct BlockStorage {
    version: TypeId,
    identifiers: IndexMap<&'static Ident, GlobalStateId>,
    metadata: &'static [&'static BlockMetadata],
}

impl BlockStorage {
    /// Build a new [`BlockStorage`] for the given [`BlockVersion`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that all provided [`BlockMetadata`] is valid for
    /// this [`Version`], and has a matching entry per-[`GlobalStateId`].
    ///
    /// # Panics
    ///
    /// Panics in `debug` builds if the provided [`BlockMetadata`] is invalid.
    #[must_use]
    pub unsafe fn build<V: BlockVersion>(metadata: &'static [&'static BlockMetadata]) -> Self {
        let mut identifiers = IndexMap::with_capacity_and_hasher(1024, <_>::default());

        for (_index, meta) in metadata.iter().enumerate() {
            #[cfg(debug_assertions)]
            #[expect(clippy::used_underscore_binding, reason = "Debug assertions")]
            Self::assert_block::<V>(_index, meta);

            match identifiers.entry(meta.identifier()) {
                Entry::Vacant(entry) => _ = entry.insert(meta.global_id_default()),
                Entry::Occupied(entry) => debug_assert_eq!(*entry.get(), meta.global_id_default()),
            }
        }

        Self { version: TypeId::of::<V>(), identifiers, metadata }
    }

    #[cfg(debug_assertions)]
    fn assert_block<V: BlockVersion>(index: usize, meta: &BlockMetadata) {
        let min = meta.global_id_base().into_usize();
        let max = min + usize::from(meta.state_count());

        debug_assert!(
            (min..=max).contains(&index),
            "GlobalStateId `{index}` is out of the expected bounds for {:?}: [{min}, {max}]",
            meta.identifier()
        );

        debug_assert!(
            meta.is_version::<V>(),
            "BlockMetadata Version mismatch for {:?}: expected {:?}",
            meta.identifier(),
            core::any::type_name::<V>()
        );
    }

    /// Get the default [`Block`] for a given [`GlobalBlockId`].
    ///
    /// # Note
    ///
    /// This is not the same as the [`GlobalStateId`]!
    ///
    /// This is the index of the [`BlockType`](crate::block::BlockType),
    /// determined by the order the blocks are stored in.
    ///
    /// This is typically used by the registry.
    #[must_use]
    pub fn get_block_by_id(&self, id: GlobalBlockId) -> Option<Block> {
        self.identifiers.get_index(id.into_usize()).and_then(|(_, id)| self.get_block_by_state(*id))
    }

    /// Get the [`Block`] for a given [`GlobalStateId`].
    ///
    /// # Note
    ///
    /// This is typically used by the world.
    #[must_use]
    pub fn get_block_by_state(&self, id: GlobalStateId) -> Option<Block> {
        let metadata = self.metadata.get(id.into_usize())?;
        let state = id.into_inner().saturating_sub(metadata.global_id_base().into_inner());
        let state = u16::try_from(state).ok()?;

        if state < metadata.state_count() {
            // SAFETY: We just checked if the state is valid for this metadata.
            Some(unsafe { Block::new_unchecked(RelativeStateId::new(state), metadata) })
        } else {
            core::hint::cold_path();
            None
        }
    }

    /// Get the default [`Block`] for a given [`Identifier`].
    ///
    /// # Note
    ///
    /// This is typically used by the inventory and registry.
    #[must_use]
    pub fn get_block_by_identifier(&self, identifier: &Ident) -> Option<Block> {
        self.identifiers.get(identifier).and_then(|id| self.get_block_by_state(*id))
    }

    /// Get the [`BlockMetadata`] of this [`BlockStorage`].
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &[&'static BlockMetadata] { self.metadata }

    /// Get the [`TypeId`] of the [`Version`] this storage is for.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version }
}
