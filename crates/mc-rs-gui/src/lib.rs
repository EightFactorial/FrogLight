use bevy::prelude::*;
use menus::MenusNodeComponent;

pub mod menus;
pub mod resources;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        resources::setup(app);

        // Setup menus
        MenusNodeComponent::setup(app);
    }
}
