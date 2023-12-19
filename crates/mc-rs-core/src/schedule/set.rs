use bevy::prelude::*;

use super::state::ApplicationState;

/// A system set that runs when the [ApplicationState] is
/// [Loading](ApplicationState::Loading)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct LoadingSet;

/// A system set that runs when the [ApplicationState] is either
/// [Loading](ApplicationState::Loading) or [MainMenu](ApplicationState::MainMenu)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MenuSet;

/// A system set that runs when the [ApplicationState] is
/// [InGame](ApplicationState::InGame)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct GameSet;

/// Adds the application state and system sets to the app
pub(super) fn configure(app: &mut App) {
    app.configure_sets(
        Update,
        (
            LoadingSet.run_if(in_state(ApplicationState::Loading)),
            MenuSet.run_if(
                in_state(ApplicationState::Loading).or_else(in_state(ApplicationState::MainMenu)),
            ),
            GameSet.run_if(in_state(ApplicationState::InGame)),
        ),
    );
}
