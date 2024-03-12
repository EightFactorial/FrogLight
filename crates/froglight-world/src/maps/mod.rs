//! A series of maps storing the world's chunks and their entities.

use bevy_app::App;

mod chunk_entity;
pub use chunk_entity::ChunkEntity;

mod chunk_map;
pub use chunk_map::WorldChunkMap;

mod world_map;
pub use world_map::WorldMap;

mod world_type;
pub use world_type::WorldType;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ChunkEntity>()
        .register_type::<WorldChunkMap>()
        .register_type::<WorldType>()
        .register_type::<WorldMap>();
}
