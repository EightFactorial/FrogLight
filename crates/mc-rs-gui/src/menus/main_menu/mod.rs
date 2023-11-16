use bevy::prelude::*;

use super::MenuComponent;

mod block_bg;

mod joining;
use joining::JoiningMenu;

mod main;
use main::MainMenu;

mod multiplayer;
use multiplayer::MultiplayerMenu;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenuRoot;

impl MenuComponent for MainMenuRoot {
    fn add_systems(app: &mut App) {
        // TODO: Add systems

        MainMenu::add_systems(app);
        MultiplayerMenu::add_systems(app);
        JoiningMenu::add_systems(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building {}", std::any::type_name::<Self>());

        // Spawn MenuComponent
        let entity = world.spawn(Self).id();
        world.entity_mut(parent).add_child(entity);

        // Build main menu
        MainMenu::build(entity, world);
        MultiplayerMenu::build(entity, world);
        JoiningMenu::build(entity, world);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub(super) enum MainMenuState {
    #[default]
    Main,
    Multiplayer,
    Joining,
    #[allow(dead_code)]
    Settings,
}

/// A trait that represents a component of the main menu.
trait MainMenuComponent: MenuComponent {
    /// The background component for this menu.
    type Background: MenuComponent;
    /// The state required for this menu to be visible.
    const STATE: MainMenuState;
}
