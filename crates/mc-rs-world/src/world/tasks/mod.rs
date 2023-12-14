use bevy::prelude::*;
use mc_rs_core::schedule::state::ApplicationState;

mod chunk_decode;
pub(super) use chunk_decode::DecodeResult;
pub use chunk_decode::{ChunkDecodeError, DecodeChunkTask};

#[cfg(feature = "shaders")]
mod chunk_material;
#[cfg(feature = "shaders")]
pub use chunk_material::ChunkMaterialTask;

use super::Chunk;

pub(super) fn setup(app: &mut App) {
    app.add_systems(
        PreUpdate,
        (
            DecodeChunkTask::poll_tasks.run_if(any_with_component::<DecodeChunkTask>()),
            #[cfg(feature = "shaders")]
            (
                ChunkMaterialTask::chunk_updater.run_if(any_with_component::<Chunk>()),
                ChunkMaterialTask::poll_tasks.run_if(any_with_component::<ChunkMaterialTask>()),
            )
                .chain(),
        )
            .run_if(in_state(ApplicationState::InGame)),
    );
}
