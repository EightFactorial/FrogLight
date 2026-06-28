//! TODO

use alloc::boxed::Box;
#[cfg(feature = "nightly")]
use alloc::{alloc::Allocator, vec::Vec};
use core::any::TypeId;

use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use indexmap::{IndexMap, map::Entry};

use crate::{
    block::{Block, BlockMetadata},
    prelude::BlockVersion,
    state::{GlobalBlockId, GlobalStateId, RelativeStateId},
};

/// A container for block data storage.
#[derive(Debug, Clone)]
pub struct BlockStorage {
    version: TypeId,
    identifiers: IndexMap<Identifier<'static>, GlobalStateId, RandomState>,
    #[cfg(not(feature = "nightly"))]
    metadata: Box<[&'static BlockMetadata]>,
    #[cfg(feature = "nightly")]
    metadata: Box<[&'static BlockMetadata], &'static (dyn Allocator + Send + Sync)>,
}

impl BlockStorage {
    /// Build a new [`BlockStorage`] for the given [`BlockVersion`].
    /// # Safety
    ///
    /// The caller must ensure that all provided block metadata has the correct
    /// global ids for this collection.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "There will never be u32::MAX blocks")]
    pub unsafe fn build<V: BlockVersion>(metadata: Box<[&'static BlockMetadata]>) -> Self {
        let mut identifiers = IndexMap::with_capacity_and_hasher(1024, RandomState::default());

        for (index, meta) in metadata.iter().enumerate() {
            if let Entry::Vacant(entry) = identifiers.entry(meta.identifier().reborrow()) {
                entry.insert_entry(GlobalStateId::new(index as u32));
            }
        }

        #[cfg(feature = "nightly")]
        let metadata = {
            let mut realloc = Vec::<_, &'static (dyn Allocator + Send + Sync)>::with_capacity_in(
                metadata.len(),
                &alloc::alloc::Global,
            );

            realloc.extend(metadata);
            realloc.into_boxed_slice()
        };

        Self { version: TypeId::of::<V>(), identifiers, metadata }
    }

    /// Build a new [`BlockStorage`] for the given [`BlockVersion`].
    ///
    /// This will use the provided allocator for the blockstate metadata slice,
    /// which will be quite large.
    ///
    /// [`V26_1`](froglight_common::version::V26_1) has about 30,000
    /// blockstates, which is 240 kB of memory (120kB on 32-bit platforms).
    ///
    /// # Safety
    ///
    /// The caller must ensure that all provided block metadata has the correct
    /// global ids for this collection.
    #[must_use]
    #[cfg(feature = "nightly")]
    #[expect(clippy::cast_possible_truncation, reason = "There will never be u32::MAX blocks")]
    pub unsafe fn build_using<V: BlockVersion, A: Allocator, A2: Allocator + Send + Sync>(
        metadata: Box<[&'static BlockMetadata], A>,
        allocator: &'static A2,
    ) -> Self {
        let mut identifiers = IndexMap::with_capacity_and_hasher(1024, RandomState::default());

        for (index, meta) in metadata.iter().enumerate() {
            if let Entry::Vacant(entry) = identifiers.entry(meta.identifier().reborrow()) {
                entry.insert_entry(GlobalStateId::new(index as u32));
            }
        }

        let metadata = {
            let mut realloc = Vec::<_, &'static (dyn Allocator + Send + Sync)>::with_capacity_in(
                metadata.len(),
                allocator,
            );

            realloc.extend(metadata);
            realloc.into_boxed_slice()
        };

        Self { version: TypeId::of::<V>(), identifiers, metadata }
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
        self.identifiers
            .get_index(id.into_inner() as usize)
            .and_then(|(_, id)| self.get_block_by_state(*id))
    }

    /// Get the [`Block`] for a given [`GlobalStateId`].
    ///
    /// # Note
    ///
    /// This is typically used when reading/writing to the world.
    #[must_use]
    pub fn get_block_by_state(&self, id: GlobalStateId) -> Option<Block> {
        let metadata = self.metadata.get(id.into_inner() as usize)?;
        let state = id.into_inner().saturating_sub(metadata.base_id().into_inner());
        let state = RelativeStateId::new(u16::try_from(state).ok()?);

        if state.into_inner() < metadata.state_count() {
            // SAFETY: We just checked if the state is valid for this metadata.
            Some(unsafe { Block::new_unchecked(state, metadata) })
        } else {
            None
        }
    }

    /// Get the default [`Block`] for a given [`Identifier`].
    ///
    /// # Note
    ///
    /// This is typically used by the inventory and registry.
    #[must_use]
    pub fn get_block_by_identifier(&self, identifier: &Identifier<'_>) -> Option<Block> {
        self.identifiers.get(identifier).and_then(|id| self.get_block_by_state(*id))
    }

    /// Get the [`TypeId`] of the [`Version`] this storage is for.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version }
}
