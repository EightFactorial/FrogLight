use bevy::prelude::*;

use crate::menus::traits::{MenuComponent, VisibilityFromWorld};

use super::{block_bg::BlockBackground, MainMenuComponent, MainMenuState};

mod list;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub(crate) struct MultiplayerMenu;

impl MainMenuComponent for MultiplayerMenu {
    type Background = BlockBackground;
    const STATE: MainMenuState = MainMenuState::Multiplayer;
}

impl MenuComponent for MultiplayerMenu {
    fn setup(app: &mut App) {
        app.add_systems(OnEnter(<Self as MainMenuComponent>::STATE), Self::show);
        app.add_systems(OnExit(<Self as MainMenuComponent>::STATE), Self::hide);

        <Self as MainMenuComponent>::Background::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MultiplayerMenu");

        // Create node
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: world.get_visibility(MainMenuState::Multiplayer),
            ..Default::default()
        };

        // Spawn MenuComponent
        let entity = world.spawn((MultiplayerMenu, node)).id();
        world.entity_mut(parent).add_child(entity);

        // Build background
        <Self as MainMenuComponent>::Background::build(parent, world);
    }
}
