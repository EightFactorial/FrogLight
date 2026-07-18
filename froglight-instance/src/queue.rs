//! TODO

use alloc::vec::Vec;

use bevy_ecs::{component::Component, reflect::ReflectComponent, world::EntityWorldMut};
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

    /// Queue a block edit to be applied to a [`Chunk`].
    pub fn queue(&mut self, position: BlockPos, block: Block) {
        let chunk = position.into_chunk_pos();
        let queue = self.queue.entry(chunk).or_default();
        queue.push(BlockEdit { position, block });
        self.is_empty = false;
    }

    /// Apply queued block edits to a [`SessionInstance`]'s [`SharedChunk`]s.
    ///
    /// # Errors
    ///
    /// Returns an error if the entity does not have a [`BlockEditQueue`] or
    /// [`SessionInstance`] component.
    #[expect(clippy::result_unit_err, reason = "TODO")]
    pub fn apply(mut entity: EntityWorldMut<'_>) -> Result<(), ()> {
        // Take the `BlockEditQueue` from the `Entity`.
        let Some(queue) = entity.get_mut::<BlockEditQueue>() else { return Err(()) };
        let mut queue = core::mem::take(queue.into_inner());

        // Map the `ChunkPos` to `Entity` of the chunks to be edited
        let mut entities = Vec::with_capacity(8);
        let Some(instance) = entity.get::<SessionInstance>() else { return Err(()) };
        for chunk in queue.queue.keys() {
            if let Some(entity) = instance.query_chunk(chunk) {
                entities.push((*chunk, entity));
            }
        }

        // Apply the queued block edits to the chunks
        entity.world_scope(|world| {
            for (chunk, edits) in &mut queue.queue {
                // Get the `SharedChunk`
                let Some((_, entity)) = entities.iter().find(|(pos, _)| chunk == pos) else {
                    continue;
                };
                let Some(mut shared) = world.get_mut::<SharedChunk>(*entity) else { continue };

                // Apply the block edits
                let mut chunk = shared.clone_inner();
                for BlockEdit { position, block } in edits.drain(..) {
                    chunk.set_block(position, block);
                }
                shared.store(chunk);
            }
        });

        // Mark the queue as empty
        queue.is_empty = true;

        // Store the `BlockEditQueue` back into the `Entity`.
        if let Some(mut q) = entity.get_mut::<BlockEditQueue>() {
            *q = queue;
        }

        Ok(())
    }
}
