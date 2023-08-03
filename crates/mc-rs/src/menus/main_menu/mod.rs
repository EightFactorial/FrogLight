use bevy::prelude::*;

use crate::systems::states::{
    application::MenuSet,
    menu::{MenuMainSet, MenuState},
};

use super::MenuRoot;

pub mod backgrounds;

/// Set up the main menu
pub(super) fn setup_menu(app: &mut App) {
    backgrounds::setup_backgrounds(app);

    app.add_systems(
        Update,
        MainMenuRoot::create
            .run_if(not(any_with_component::<MainMenuRoot>()))
            .in_set(MenuSet),
    );

    app.add_systems(
        OnEnter(MenuState::Main),
        MainMenuRoot::show
            .run_if(any_with_component::<MainMenuRoot>())
            .in_set(MenuMainSet),
    );
    app.add_systems(
        OnExit(MenuState::Main),
        MainMenuRoot::hide
            .run_if(any_with_component::<MainMenuRoot>())
            .in_set(MenuMainSet),
    );
}

/// A marker component for the main menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MainMenuRoot;

impl MainMenuRoot {
    /// Create the main menu
    fn create(
        query: Query<Entity, With<MenuRoot>>,
        state: Res<State<MenuState>>,
        mut commands: Commands,
    ) {
        let visibility = if **state == MenuState::Main {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        let entity = commands
            .spawn((
                MainMenuRoot,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.),
                        top: Val::Px(0.),
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..Default::default()
                    },
                    visibility,
                    background_color: Color::NONE.into(),
                    ..Default::default()
                },
            ))
            .id();
        commands.entity(query.single()).add_child(entity);
    }

    /// Make the menu visible
    pub fn show(mut vis: Query<&mut Visibility, With<MainMenuRoot>>) {
        *vis.single_mut() = Visibility::Visible;
    }

    /// Make the menu visible
    pub fn hide(mut vis: Query<&mut Visibility, With<MainMenuRoot>>) {
        *vis.single_mut() = Visibility::Hidden;
    }
}
