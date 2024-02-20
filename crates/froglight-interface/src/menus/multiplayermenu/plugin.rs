use bevy::prelude::*;

/// A plugin that adds a multiplayer menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceMultiplayerMenuPlugin;

impl Plugin for InterfaceMultiplayerMenuPlugin {
    fn build(&self, app: &mut App) { crate::systemset::build(app); }
}
