use bevy::prelude::*;

use super::traits::MenuComponent;

mod background;

mod menu;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct SettingsMenuRoot;

impl MenuComponent for SettingsMenuRoot {
    fn add_systems(_app: &mut App) {
        // TODO: Add systems
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building SettingsMenuRoot");

        // Spawn MenuComponent
        let entity = world.spawn(Self).id();
        world.entity_mut(parent).add_child(entity);

        // TODO: Build SettingsMenu, etc.
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
enum SettingsMenuState {
    #[default]
    Overview,
    Video,
    Audio,
    Controls,
    ResourcePacks,
}

/// A trait that represents a component of the main menu.
trait SettingsMenuComponent: MenuComponent {
    /// The background component for this menu.
    type Background: MenuComponent;
    /// The state required for this menu to be visible.
    const STATE: SettingsMenuState;
}
