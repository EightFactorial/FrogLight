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
                Outline {
                    width: Val::Px(4.0),
                    color: Color::BLACK,
                    ..Default::default()
                },
            ))
            .with_children(|node| {
                // Create a solid white border around the node.
                node.spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(4.0)),
                        ..Default::default()
                    },
                    border_color: Color::WHITE.into(),
                    ..Default::default()
                });
                // Create a mostly solid dark gray border for the right and bottom
                node.spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::new(Val::Px(0.0), Val::Px(4.0), Val::Px(0.0), Val::Px(4.0)),
                        ..Default::default()
                    },
                    border_color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
                    ..Default::default()
                });
                // Create a mostly solid light gray border for the left and top
                node.spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::new(Val::Px(4.0), Val::Px(0.0), Val::Px(4.0), Val::Px(0.0)),
                        ..Default::default()
                    },
                    border_color: Color::rgba(0.7, 0.7, 0.7, 0.8).into(),
                    ..Default::default()
                });
            })
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
