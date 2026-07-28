//! TODO

use alloc::vec::Vec;

use bevy_ecs::{
    component::Component, query::Without, reflect::ReflectComponent, resource::IsResource,
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
    /// # Note
    ///
    /// If you're having issues with `chunk`,
    /// you can use [`Query::reborrow`] to obtain ownership
    /// and [`Query::transmute_lens`] to change it's values.
    pub fn apply(
        &mut self,
        instance: &SessionInstance,
        mut chunks: Query<&mut SharedChunk, Without<IsResource>>,
    ) {
        for (chunk, edits) in &mut self.queue {
            if let Some(entity) = instance.query_chunk(chunk)
                && let Ok(mut shared) = chunks.get_mut(entity)
            {
                // Clone, apply edits, and replace the existing chunk.
                let mut chunk = shared.clone_inner();
                for BlockEdit { position, block } in edits.drain(..) {
                    chunk.set_block(position, block);
                }
                shared.store(chunk);
            } else {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    target: "froglight_instance",
                    "Could not find Chunk ({}, {}), dropping edits.",
                    chunk.x(),
                    chunk.z(),
                );

                // Drop edits for missing chunks.
                edits.clear();
            }
        }
        self.is_empty = true;
    }
}
