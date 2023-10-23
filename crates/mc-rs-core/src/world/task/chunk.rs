use bevy::tasks::AsyncComputeTaskPool;

use crate::{
    blocks::BlockData,
    world::{structure::chunk::ChunkSections, SECTION_COUNT},
};

use super::{section::section_fn, ChunkTaskResult};

/// Generates a mesh for all sections in a chunk
pub(super) async fn chunk_fn(
    chunk: ChunkSections,
    neighbors: [Option<ChunkSections>; 4],
    block_data: BlockData,
) -> ChunkTaskResult {
    let pool = AsyncComputeTaskPool::get();

    let mut results = Vec::with_capacity(SECTION_COUNT);
    let mut tasks = Vec::with_capacity(SECTION_COUNT);

    for index in 0..SECTION_COUNT {
        // If the section is empty, skip it
        if let Some(chunk) = chunk.read().get(index) {
            if chunk.block_count == 0 {
                tasks.push(None);
                continue;
            }
        }

        let neighbors = [
            neighbors[0].as_ref().map(|c| c.read()[index].get_blocks()),
            neighbors[1].as_ref().map(|c| c.read()[index].get_blocks()),
            neighbors[2].as_ref().map(|c| c.read()[index].get_blocks()),
            neighbors[3].as_ref().map(|c| c.read()[index].get_blocks()),
            if index > 0 {
                Some(chunk.read()[index - 1].get_blocks())
            } else {
                None
            },
            if index < SECTION_COUNT - 1 {
                Some(chunk.read()[index + 1].get_blocks())
            } else {
                None
            },
        ];

        // Spawn a new thread for the section
        let task = Some(pool.spawn(section_fn(
            chunk.read()[index].get_blocks(),
            neighbors,
            block_data.clone(),
        )));

        tasks.push(task);
    }

    // Wait for all sections to finish
    for task in tasks {
        match task {
            Some(task) => results.push(task.await),
            None => results.push(None),
        }
    }

    results
}
