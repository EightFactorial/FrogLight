//! TODO

use bevy_app::{App, Plugin};

mod chunk_map;
pub use chunk_map::ChunkMap;

/// A [`Plugin`] that adds various world-related systems and components.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) { chunk_map::build(app); }
}
