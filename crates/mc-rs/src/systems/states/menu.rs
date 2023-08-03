use bevy::prelude::*;

use super::application::{ApplicationState, InMenuSet};

/// The current state of the application
///
/// This is used to determine which systems should be run
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum MenuState {
    #[default]
    None,
    Main,
    Settings,
    Server,
    Credits,
}

/// A system set that runs when the [MenuState] is
/// [MainMenu](MenuState::MainMenu) state
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MenuMainSet;

/// A system set that runs when the [MenuState] is
/// [SettingsMenu](MenuState::SettingsMenu) state
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MenuSettingsSet;

/// A system set that runs when the [MenuState] is
/// [ServerMenu](MenuState::ServerMenu) state
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MenuServerSet;

/// A system set that runs when the [MenuState] is
/// [CreditsMenu](MenuState::CreditsMenu) state
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MenuCreditsSet;

/// Adds the application state to the app
pub(super) fn add_state(app: &mut App) {
    app.add_state::<MenuState>();

    app.configure_sets(
        Update,
        (
            MenuMainSet
                .run_if(in_state(MenuState::Main))
                .in_set(InMenuSet),
            MenuSettingsSet
                .run_if(in_state(MenuState::Settings))
                .in_set(InMenuSet),
            MenuServerSet
                .run_if(in_state(MenuState::Server))
                .in_set(InMenuSet),
            MenuCreditsSet
                .run_if(in_state(MenuState::Credits))
                .in_set(InMenuSet),
        ),
    );

    app.add_systems(
        OnEnter(ApplicationState::InMenu),
        set_state.in_set(InMenuSet),
    );
    app.add_systems(
        OnExit(ApplicationState::InMenu),
        clear_state.in_set(InMenuSet),
    );
}

fn set_state(mut state: ResMut<NextState<MenuState>>) { state.set(MenuState::Main); }

fn clear_state(mut state: ResMut<NextState<MenuState>>) { state.set(MenuState::None); }
