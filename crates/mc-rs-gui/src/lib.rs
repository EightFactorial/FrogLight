use bevy::prelude::*;

pub mod menus;
use menus::{loading::LoadingMenuRoot, MenuRoot};

pub mod assets;
pub mod resources;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        resources::setup(app);
        assets::setup(app);

        // Setup menus
        MenuRoot::setup(app);
        LoadingMenuRoot::setup(app);
    }
}
