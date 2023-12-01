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
        app.add_systems(
            OnEnter(<Self as MainMenuComponent>::STATE),
            (Self::show, <Self as MainMenuComponent>::Background::show),
        );
        app.add_systems(
            OnExit(<Self as MainMenuComponent>::STATE),
            (Self::hide, <Self as MainMenuComponent>::Background::hide),
        );

        app.add_systems(
            Update,
            esc_pressed.run_if(in_state(MainMenuState::Multiplayer)),
        );

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
        world.spawn((MultiplayerMenu, node)).set_parent(parent);
    }
}

fn esc_pressed(input: Res<Input<KeyCode>>, mut state: ResMut<NextState<MainMenuState>>) {
    if input.just_pressed(KeyCode::Escape) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Esc pressed, returning to MainMenu");

        state.set(MainMenuState::Main);
    }
}
