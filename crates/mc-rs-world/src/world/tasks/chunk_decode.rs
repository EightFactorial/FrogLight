use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use derive_more::{From, Into};
use futures_lite::future;
use mc_rs_protocol::types::packets::chunk_data::ChunkDataPacket;
use thiserror::Error;

use crate::{resources::Worlds, world::Chunk};

#[derive(Debug, Error)]
pub enum ChunkDecodeError {
    #[error("Invalid Palette data")]
    InvalidPalette,
    #[error("Invalid Container data")]
    InvalidContainer,
    #[error("Invalid Section data")]
    InvalidSection,
}

/// A [Task] that decodes a [`Chunk`] from a [`ChunkDataPacket`].
///
/// This is used to decode chunks in parallel.
#[derive(Debug, From, Into, Deref, DerefMut, Component)]
pub struct DecodeChunkTask(Task<DecodeResult>);
pub(crate) type DecodeResult = Result<Chunk, ChunkDecodeError>;

impl DecodeChunkTask {
    pub fn create(data: ChunkDataPacket) -> Self {
        let pool = AsyncComputeTaskPool::get();
        let task = pool.spawn(Chunk::decode_chunk(data));
        Self(task)
    }

    /// A [bevy] [System] that polls any [`DecodeChunkTask`]s that have
    /// been created and inserts the decoded [`Chunk`]s.
    pub(crate) fn poll_tasks(
        mut query: Query<(Entity, &mut DecodeChunkTask)>,
        mut worlds: ResMut<Worlds>,
        mut commands: Commands,
    ) {
        query.iter_mut().for_each(|(entity, mut task)| {
            if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
                let mut commands = commands.entity(entity);

                match result {
                    Ok(chunk) => {
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        trace!("Decoded Chunk {entity:?}");

                        commands.insert(chunk);
                        commands.remove::<DecodeChunkTask>();
                    }
                    Err(err) => {
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        let mut found = false;

                        // Loop through all worlds and check if the
                        // chunk entity exists in any of them
                        'world: for (_, world) in worlds.iter_mut() {
                            for (chunk_pos, chunk_entity) in world.clone().into_iter() {
                                if chunk_entity == entity {
                                    #[cfg(any(debug_assertions, feature = "debug"))]
                                    {
                                        error!("Error decoding Chunk {entity:?}: {err}");
                                        found = true;
                                    }

                                    world.remove_entity(&chunk_pos);
                                    break 'world;
                                }
                            }
                        }

                        // Emit a warning if the chunk entity was not found in any world
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        if !found {
                            error!("Error decoding Chunk {entity:?}: {err}");
                            warn!("Chunk {entity:?} not found in any world?");
                        }

                        commands.despawn_recursive();
                    }
                }
            }
        });
    }
}
