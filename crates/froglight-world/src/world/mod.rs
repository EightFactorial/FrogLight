//! A [`World`], consisting of a set of [`Chunks`](Chunk).
use bevy::app::App;

pub mod tasks;

mod section;
pub use section::Section;

mod chunk;
pub use chunk::{Chunk, HeightMaps};

mod container;
pub use container::{BiomeContainer, BlockContainer, ChunkDataContainer, HeightMapContainer};

mod palette;
pub use palette::Palette;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<Section>()
        .register_type::<HeightMaps>()
        .register_type::<Palette>()
        .register_type::<BiomeContainer>()
        .register_type::<BlockContainer>()
        .register_type::<ChunkDataContainer<BlockContainer>>()
        .register_type::<ChunkDataContainer<BiomeContainer>>()
        .register_type::<HeightMapContainer>();
}
