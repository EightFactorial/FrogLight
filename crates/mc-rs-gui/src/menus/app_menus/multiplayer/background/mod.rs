use bevy::prelude::*;

use crate::menus::{
    app_menus::states::MainMenuState, states::menus::MenuComponentMenusSet, traits::MenuComponent,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BackgroundNodeComponent;

impl MenuComponent for BackgroundNodeComponent {
    fn setup(app: &mut App) {
        app.add_systems(
            Update,
            Self::pressed_escape
                .in_set(MenuComponentMenusSet)
                .run_if(in_state(MainMenuState::Multiplayer)),
        );
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building BackgroundNodeComponent");

        // Create the node.
        world
            .spawn((
                BackgroundNodeComponent,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..Default::default()
                },
            ))
            .set_parent(parent);
    }
}

impl BackgroundNodeComponent {
    fn pressed_escape(input: Res<Input<KeyCode>>, mut state: ResMut<NextState<MainMenuState>>) {
        if input.just_pressed(KeyCode::Escape) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            trace!("Pressed Escape");

            state.set(MainMenuState::MainMenu);
        }
    }
}
