use bevy::prelude::*;

pub mod menus;
use menus::MenuRoot;

pub mod assets;
pub mod resources;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        resources::setup(app);
        assets::setup(app);

        MenuRoot::add_systems(app);
    }
}
