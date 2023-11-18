use bevy::prelude::*;

use super::traits::MenuComponent;

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
    fn setup(app: &mut App) {
        app.add_state::<MainMenuState>();

        MainMenu::setup(app);
        MultiplayerMenu::setup(app);
        JoiningMenu::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenuRoot");

        // Create node
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        };

        // Spawn MenuComponent
        let entity = world.spawn((MainMenuRoot, node)).id();
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
    Settings,
}

/// A trait that represents a component of the main menu.
trait MainMenuComponent: MenuComponent {
    /// The background component for this menu.
    type Background: MenuComponent;
    /// The state required for this menu to be visible.
    const STATE: MainMenuState;
}
