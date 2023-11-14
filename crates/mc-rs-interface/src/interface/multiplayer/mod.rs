use bevy::prelude::*;
use mc_rs_core::schedule::{set::MenuSet, state::ApplicationState};

mod background;
use background::MultiplayerBackground;

use crate::traits::{interface::InterfaceComponent, world::MenuVisibility};

use super::state::MainMenuState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct MultiplayerInterface;

impl InterfaceComponent for MultiplayerInterface {
    fn setup(app: &mut App) {
        // Exit the multiplayer interface when pressing escape
        app.add_systems(
            PreUpdate,
            MultiplayerInterface::press_escape
                .run_if(in_state(MainMenuState::Multiplayer))
                .in_set(MenuSet),
        );

        // Show the multiplayer interface when entering the MainMenuState::Multiplayer state
        app.add_systems(
            OnEnter(MainMenuState::Multiplayer),
            MultiplayerInterface::show
                .run_if(
                    in_state(ApplicationState::MainMenu)
                        .and_then(any_with_component::<MultiplayerInterface>()),
                )
                .in_set(MenuSet),
        );
        // Hide the multiplayer interface when exiting the MainMenuState::Multiplayer state
        app.add_systems(
            OnExit(MainMenuState::Multiplayer),
            MultiplayerInterface::hide
                .run_if(
                    in_state(ApplicationState::MainMenu)
                        .and_then(any_with_component::<MultiplayerInterface>()),
                )
                .in_set(MenuSet),
        );

        // Add systems for interface components
        MultiplayerBackground::setup(app);
    }

    fn build(root: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MultiplayerInterface");

        // Create the multiplayer node
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

        // Spawn the multiplayer interface node as a child of the interface root
        let multiplayer = world.spawn((MultiplayerInterface, node)).id();
        world.entity_mut(root).add_child(multiplayer);

        // Build interface components
        MultiplayerBackground::build(multiplayer, world);
    }
}

impl MultiplayerInterface {
    /// Exit the multiplayer interface when pressing escape.
    fn press_escape(input: Res<Input<KeyCode>>, mut state: ResMut<NextState<MainMenuState>>) {
        if input.just_pressed(KeyCode::Escape) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Pressing Escape");

            state.set(MainMenuState::Main);
        }
    }

    /// Show the multiplayer interface.
    fn show(mut query: Query<&mut Visibility, With<MultiplayerInterface>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Showing MultiplayerInterface");

        query.iter_mut().for_each(|mut visibility| {
            *visibility = Visibility::Visible;
        });
    }

    /// Hide the multiplayer interface.
    fn hide(mut query: Query<&mut Visibility, With<MultiplayerInterface>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Hiding MultiplayerInterface");

        query.iter_mut().for_each(|mut visibility| {
            *visibility = Visibility::Hidden;
        });
    }
}
