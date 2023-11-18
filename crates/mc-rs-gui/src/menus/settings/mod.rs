use bevy::prelude::*;

use crate::menus::{main_menu::MainMenuState, traits::VisibilityFromWorld};

use super::traits::MenuComponent;

mod background;

mod menu;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct SettingsMenuRoot;

impl MenuComponent for SettingsMenuRoot {
    fn setup(_app: &mut App) {
        // TODO: Add systems
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building SettingsMenuRoot");

        let node = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: world.get_visibility(MainMenuState::Settings),
            ..Default::default()
        };

        // Spawn MenuComponent
        let entity = world.spawn((SettingsMenuRoot, node)).id();
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
