use bevy::prelude::*;

pub mod resources;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, _app: &mut App) {}
}
