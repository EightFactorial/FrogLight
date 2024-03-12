use bevy::prelude::*;

use crate::menus::panorama::MainMenuBackgroundEnable;

/// A plugin that adds a multiplayer menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceMultiplayerMenuPlugin;

impl Plugin for InterfaceMultiplayerMenuPlugin {
    fn build(&self, app: &mut App) {
        // Add `SystemSet`s
        crate::menus::build(app);
        super::systemset::build(app);
    }

    fn finish(&self, app: &mut App) {
        // Enable the panorama for the multiplayer menu
        Self::enable_panorama(&mut app.world);
    }
}

impl InterfaceMultiplayerMenuPlugin {
    /// If there is a [`MainMenuBackgroundEnable`] resource,
    /// enable the panorama for the multiplayer menu.
    fn enable_panorama(world: &mut World) {
        if let Some(mut settings) = world.get_resource_mut::<MainMenuBackgroundEnable>() {
            settings.multiplayer_menu = true;
        }
    }
}
