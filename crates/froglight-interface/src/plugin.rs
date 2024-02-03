use bevy::prelude::*;

/// A [`Plugin`] that manages menus and other GUI elements
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, _app: &mut App) {}
}
