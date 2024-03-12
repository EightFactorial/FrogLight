use bevy::prelude::*;

use crate::menus::panorama::MainMenuBackgroundEnable;

/// A plugin that adds a settings menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceSettingsMenuPlugin;

impl Plugin for InterfaceSettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        // Add `SystemSet`s
        crate::menus::build(app);
        super::systemset::build(app);
    }

    fn finish(&self, app: &mut App) {
        // Enable the panorama for the settings menu
        Self::enable_panorama(&mut app.world);
    }
}

impl InterfaceSettingsMenuPlugin {
    /// If there is a [`MainMenuBackgroundEnable`] resource,
    /// enable the panorama for the settings menu.
    fn enable_panorama(world: &mut World) {
        if let Some(mut settings) = world.get_resource_mut::<MainMenuBackgroundEnable>() {
            settings.settings_menu = true;
        }
    }
}
