use bevy::prelude::*;

/// A plugin that adds a main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceMainMenuPlugin;

impl Plugin for InterfaceMainMenuPlugin {
    fn build(&self, app: &mut App) { crate::systemset::build(app); }
}
