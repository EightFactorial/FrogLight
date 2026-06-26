//! TODO

use alloc::vec::Vec;
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
    metadata: Vec<&'static BlockMetadata>,
}

impl BlockStorage {
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

    /// Build a new [`BlockStorage`] for the given [`BlockVersion`].
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "There will never be u32::MAX blocks")]
    pub fn build<V: BlockVersion>(metadata: Vec<&'static BlockMetadata>) -> Self {
        let mut identifiers = IndexMap::with_capacity_and_hasher(1024, RandomState::default());
        for (index, meta) in metadata.iter().enumerate() {
            if let Entry::Vacant(entry) = identifiers.entry(meta.identifier().reborrow()) {
                entry.insert_entry(GlobalStateId::new(index as u32));
            }
        }

        Self { version: TypeId::of::<V>(), identifiers, metadata }
    }
}
