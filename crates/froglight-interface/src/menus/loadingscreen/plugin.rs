use bevy::prelude::*;

/// A plugin that adds a loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceLoadingScreenPlugin;

impl Plugin for InterfaceLoadingScreenPlugin {
    fn build(&self, app: &mut App) { crate::systemset::build(app); }
}
