use bevy::prelude::*;

pub mod biomes;
pub mod blocks;
pub mod entities;
pub mod resources;
pub mod world;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        resources::setup(app);
        world::setup(app);
    }

    #[cfg(feature = "shaders")]
    fn finish(&self, app: &mut App) { world::shaders::check_bindless_texture_support(app); }
}
