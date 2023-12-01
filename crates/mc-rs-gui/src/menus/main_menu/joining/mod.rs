use bevy::prelude::*;

use crate::menus::traits::{MenuComponent, VisibilityFromWorld};

use super::{block_bg::BlockBackground, MainMenuComponent, MainMenuState};

mod message;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub(crate) struct JoiningMenu;

impl MainMenuComponent for JoiningMenu {
    type Background = BlockBackground;
    const STATE: MainMenuState = MainMenuState::Joining;
}

impl MenuComponent for JoiningMenu {
    fn setup(app: &mut App) {
        app.add_systems(OnEnter(<Self as MainMenuComponent>::STATE), Self::show);
        app.add_systems(OnExit(<Self as MainMenuComponent>::STATE), Self::hide);

        <Self as MainMenuComponent>::Background::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building JoiningMenu");

        // Create node
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: world.get_visibility(MainMenuState::Joining),
            ..Default::default()
        };

        // Spawn MenuComponent
        world.spawn((JoiningMenu, node)).set_parent(parent);
    }
}
