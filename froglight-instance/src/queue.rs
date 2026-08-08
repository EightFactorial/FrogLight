//! TODO

use alloc::vec::Vec;

use bevy_ecs::{
    component::Component, entity::EntityHashMap, query::QueryFilter, reflect::ReflectComponent,
    system::Query,
};
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use foldhash::fast::RandomState;
use froglight_block::prelude::*;
use froglight_world::prelude::*;
use hashbrown::HashMap;

use crate::prelude::*;

/// A queue of [`BlockEdit`]s to be applied.
#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(opaque, Debug, Default, Clone, PartialEq, Component)]
pub struct BlockEditQueue {
    queue: HashMap<ChunkPos, Vec<BlockEdit>, RandomState>,
    is_empty: bool,
}

/// A block edit to be applied to a [`Chunk`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BlockEdit {
    position: BlockPos,
    block: Block,
}

impl Default for BlockEditQueue {
    #[inline]
    fn default() -> Self { Self::new() }
}

impl BlockEditQueue {
    /// Create a new, empty [`BlockQueue`].
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self { queue: HashMap::with_hasher(RandomState::default()), is_empty: true }
    }

    /// Returns `true` if the queue is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.is_empty }

    /// Push a block edit to be applied to a [`Chunk`].
    pub fn push(&mut self, position: BlockPos, block: Block) {
        let chunk = position.into_chunk_pos();
        let queue = self.queue.entry(chunk).or_default();
        queue.push(BlockEdit { position, block });
        self.is_empty = false;
    }

    /// Remove all queued block edits for a [`Chunk`].
    ///
    /// In case you are sent a [`Chunk`] after queuing edits for it,
    /// you can use this to remove the queued edits.
    #[inline]
    pub fn remove(&mut self, chunk: &ChunkPos) { self.queue.remove(chunk); }

    /// Apply queued block edits to a [`SessionInstance`]'s [`SharedChunk`]s.
    ///
    /// Automatically replaces the existing [`SharedChunk`]s with the modified
    /// ones.
    ///
    /// # Note
    ///
    /// If you're having issues with `chunks`,
    /// you can use [`Query::reborrow`] to obtain ownership
    /// and [`Query::transmute_lens`] to change it's values.
    pub fn apply_to<F: QueryFilter>(
        &mut self,
        instance: &SessionInstance,
        mut chunks: Query<&mut SharedChunk, F>,
    ) {
        // Skip if the queue is empty.
        if self.is_empty {
            return;
        }

        for (chunk, edits) in self.queue.iter_mut().filter(|(_, edits)| !edits.is_empty()) {
            if let Some(entity) = instance.query_chunk(chunk)
                && let Ok(mut shared) = chunks.get_mut(entity)
            {
                // Apply edits, cloning only if needed.
                let chunk = shared.make_mut();
                for BlockEdit { position, block } in edits.drain(..) {
                    chunk.set_block(position, block);
                }
            } else {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    target: "froglight_instance",
                    "Failed to apply edits to unknown Chunk ({}, {})",
                    chunk.x(),
                    chunk.z(),
                );

                // Drop edits for missing chunks.
                edits.clear();
            }
        }

        self.is_empty = true;
    }

    /// Apply queued block edits to a [`SessionInstance`]'s [`SharedChunk`]s.
    ///
    /// Returns an [`EntityHashMap`] containing the modified [`SharedChunk`]s.
    ///
    /// # Note
    ///
    /// If you're having issues with `chunks`,
    /// you can use [`Query::reborrow`] to obtain ownership
    /// and [`Query::transmute_lens`] to change it's values.
    pub fn apply_clone<F: QueryFilter>(
        &mut self,
        instance: &SessionInstance,
        chunks: Query<&SharedChunk, F>,
    ) -> EntityHashMap<SharedChunk> {
        let mut output = EntityHashMap::new();

        // Skip if the queue is empty.
        if self.is_empty {
            return output;
        }

        for (chunk, edits) in self.queue.iter_mut().filter(|(_, edits)| !edits.is_empty()) {
            if let Some(entity) = instance.query_chunk(chunk)
                && let Ok(shared) = chunks.get(entity)
            {
                // Clone, apply edits, and store the modified chunk.
                let mut chunk = shared.clone_inner();
                for BlockEdit { position, block } in edits.drain(..) {
                    chunk.set_block(position, block);
                }
                output.insert(entity, SharedChunk::new(chunk));
            } else {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    target: "froglight_instance",
                    "Failed to apply edits to unknown Chunk ({}, {})",
                    chunk.x(),
                    chunk.z(),
                );

                // Drop edits for missing chunks.
                edits.clear();
            }
        }

        self.is_empty = true;
        output
    }
}
