use bevy::prelude::*;

mod world;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) { world::setup(app); }
}
