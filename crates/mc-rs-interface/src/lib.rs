use bevy::prelude::*;

pub mod menus;
pub mod player;
pub mod settings;
pub mod util;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        player::setup(app);
        menus::setup(app);
    }
}
