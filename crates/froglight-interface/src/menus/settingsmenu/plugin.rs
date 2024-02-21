use bevy::prelude::*;

/// A plugin that adds a settings menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceSettingsMenuPlugin;

impl Plugin for InterfaceSettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        // Add `SystemSet`s
        crate::menus::build(app);
        super::systemset::build(app);
    }
}
