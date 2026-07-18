//! TODO

use bevy_app::{App, Plugin};

use crate::{
    component::{ChunkBlockPos, DimensionPos},
    prelude::*,
};

/// A [`Plugin`] that ...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BlockPos>()
            .register_type::<ChunkPos>()
            .register_type::<ChunkBlockPos>()
            .register_type::<DimensionPos>();

        #[cfg(all(feature = "froglight-biome", feature = "froglight-block"))]
        {
            app.register_type::<Chunk>().register_type::<SharedChunk>();
        }
    }
}
