use bevy::app::App;

pub mod container;
pub mod palette;
pub mod section;
pub mod tasks;

mod chunk;
pub use chunk::Chunk;

pub mod heightmap;
pub use heightmap::HeightMap;

#[cfg(feature = "shaders")]
pub mod shaders;

pub(super) fn setup(app: &mut App) {
    tasks::setup(app);

    #[cfg(feature = "shaders")]
    shaders::setup(app);
}
