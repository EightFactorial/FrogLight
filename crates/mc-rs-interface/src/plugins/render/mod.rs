use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, _app: &mut App) {}
}
