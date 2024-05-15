use bevy_app::App;
use froglight_protocol::common::{ChunkPosition, Difficulty, GameMode};

pub(super) fn build(app: &mut App) {
    // Register some protocol types for reflection.
    app.init_resource::<Difficulty>()
        .register_type::<Difficulty>()
        .register_type::<GameMode>()
        .register_type::<ChunkPosition>();
}
