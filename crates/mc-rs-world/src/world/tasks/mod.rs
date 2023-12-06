use bevy::prelude::*;
use mc_rs_core::schedule::state::ApplicationState;

pub(crate) mod decode_chunk;
pub(super) use decode_chunk::DecodeResult;
pub use decode_chunk::{ChunkDecodeError, DecodeChunkTask};

pub(super) fn setup(app: &mut App) {
    app.add_systems(
        PreUpdate,
        DecodeChunkTask::poll_tasks.run_if(
            in_state(ApplicationState::InGame).and_then(any_with_component::<DecodeChunkTask>()),
        ),
    );
}
