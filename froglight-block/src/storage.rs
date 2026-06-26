//! TODO

use alloc::vec::Vec;
use core::any::TypeId;

use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use indexmap::IndexMap;

use crate::{
    block::{Block, BlockMetadata},
    prelude::BlockVersion,
    state::{GlobalId, StateId},
};

/// A container for block data storage.
#[derive(Debug, Clone)]
pub struct BlockStorage {
    version: TypeId,
    identifiers: IndexMap<Identifier<'static>, GlobalId, RandomState>,
    metadata: Vec<&'static BlockMetadata>,
}

impl BlockStorage {
    /// Get the [`Block`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_block(&self, id: GlobalId) -> Option<Block> {
        let metadata = self.get_metadata(id)?;
        let state = id.into_inner().saturating_sub(metadata.base_id().into_inner());
        let state = StateId::new(u16::try_from(state).ok()?);

        if state.into_inner() < metadata.state_count() {
            // SAFETY: We just checked if the state is valid for this metadata.
            Some(unsafe { Block::new_unchecked(state, metadata) })
        } else {
            None
        }
    }

    /// Get the [`Block`] for a given [`Identifier`].
    #[must_use]
    pub fn get_block_by_identifier(&self, identifier: &Identifier<'_>) -> Option<Block> {
        self.identifiers.get(identifier).and_then(|id| self.get_block(*id))
    }

    /// Get the [`Block`] for a given block ID.
    ///
    /// # Note
    ///
    /// This is not the same as the [`GlobalId`]!
    ///
    /// This is the index of the [`BlockType`](crate::block::BlockType),
    /// determined by the order the blocks are stored in.
    ///
    /// This is typically used by the registry.
    #[must_use]
    pub fn get_block_by_id(&self, id: u32) -> Option<Block> {
        let mut count = 0u32;
        let mut index = 0usize;
        while let Some(&metadata) = self.metadata.get(index) {
            if count == id {
                return self.get_block(metadata.default_id());
            }
            count += 1;
            index += usize::from(metadata.state_count());
        }
        None
    }

    /// Get the [`BlockMetadata`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_metadata(&self, id: GlobalId) -> Option<&'static BlockMetadata> {
        self.metadata.get(id.into_inner() as usize).copied()
    }

    /// Get the [`TypeId`] of the [`Version`] this storage is for.
    #[inline]
    #[must_use]
    pub const fn version(&self) -> TypeId { self.version }

    /// Build a new [`BlockStorage`] for the given [`BlockVersion`].
    #[must_use]
    pub fn build<V: BlockVersion>(metadata: Vec<&'static BlockMetadata>) -> Self {
        let identifiers = IndexMap::with_capacity_and_hasher(1024, RandomState::default());

        Self { version: TypeId::of::<V>(), identifiers, metadata }
    }
}
