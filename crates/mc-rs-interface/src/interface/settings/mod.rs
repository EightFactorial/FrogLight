use bevy::prelude::*;
use mc_rs_core::schedule::{set::MenuSet, state::ApplicationState};

use crate::{
    interface::state::MainMenuState,
    traits::{interface::InterfaceComponent, world::MenuVisibility},
};

use super::state::SettingsState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct SettingsInterface;

impl InterfaceComponent for SettingsInterface {
    fn setup(app: &mut App) {
        // Exit the multiplayer interface when pressing escape
        app.add_systems(
            PreUpdate,
            SettingsInterface::press_escape
                .run_if(in_state(MainMenuState::Settings))
                .in_set(MenuSet),
        );

        // Show the multiplayer interface when entering the MainMenuState::Settings state
        app.add_systems(
            OnEnter(MainMenuState::Settings),
            SettingsInterface::show
                .run_if(any_with_component::<SettingsInterface>())
                .in_set(MenuSet),
        );
        // Hide the multiplayer interface when exiting the MainMenuState::Settings state
        app.add_systems(
            OnExit(MainMenuState::Settings),
            SettingsInterface::hide
                .run_if(any_with_component::<SettingsInterface>())
                .in_set(MenuSet),
        );

        // TODO: Add systems for interface components
    }

    fn build(root: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building SettingsInterface");

        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,

                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                ..Default::default()
            },
            background_color: BackgroundColor(Color::GRAY),
            visibility: world
                .get_menu_visibility(ApplicationState::MainMenu, MainMenuState::Multiplayer),
            ..Default::default()
        };

        let background = world.spawn((SettingsInterface, node)).id();
        world.entity_mut(root).add_child(background);

        // TODO: Build interface components
    }
}

impl SettingsInterface {
    /// Either return to the settings overview or the main menu when pressing escape.
    fn press_escape(
        input: Res<Input<KeyCode>>,
        state: ResMut<State<SettingsState>>,

        mut settings_state: ResMut<NextState<SettingsState>>,
        mut menu_state: ResMut<NextState<MainMenuState>>,
    ) {
        if input.just_pressed(KeyCode::Escape) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Pressing Escape");

            match **state {
                SettingsState::Overview => menu_state.set(MainMenuState::Main),
                _ => settings_state.set(SettingsState::Overview),
            }
        }
    }

    /// Show the multiplayer interface.
    fn show(mut query: Query<&mut Visibility, With<SettingsInterface>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Showing SettingsInterface");

        query.iter_mut().for_each(|mut visibility| {
            *visibility = Visibility::Visible;
        });
    }

    /// Hide the multiplayer interface.
    fn hide(mut query: Query<&mut Visibility, With<SettingsInterface>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Hiding SettingsInterface");

        query.iter_mut().for_each(|mut visibility| {
            *visibility = Visibility::Hidden;
        });
    }
}
