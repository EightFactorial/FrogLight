//! Utilities for tracking entities and chunks.

mod chunkmap;
pub use chunkmap::ChunkPositionMap;

mod entitymap;
pub use entitymap::EntityChunkMap;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    chunkmap::build(app);
    entitymap::build(app);
}
