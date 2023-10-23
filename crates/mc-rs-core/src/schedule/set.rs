use bevy::prelude::*;

use super::state::ApplicationState;

/// A system set that runs when the [ApplicationState] is either
/// [InMenu](ApplicationState::InMenu) or [SplashScreen](ApplicationState::SplashScreen)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MenuSet;

/// A system set that runs when the [ApplicationState] is
/// [InMenu](ApplicationState::InMenu)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct InMenuSet;

/// A system set that runs when the [ApplicationState] is
/// [Game](ApplicationState::Game)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct GameSet;

/// Adds the application state and system sets to the app
pub(super) fn configure(app: &mut App) {
    app.configure_sets(
        Update,
        (
            // Splash and MainMenu
            #[cfg(feature = "splash")]
            {
                MenuSet.run_if(
                    in_state(ApplicationState::InMenu)
                        .or_else(in_state(ApplicationState::SplashScreen)),
                )
            },
            #[cfg(not(feature = "splash"))]
            {
                MenuSet.run_if(in_state(ApplicationState::InMenu))
            },
            InMenuSet
                .run_if(in_state(ApplicationState::InMenu))
                .in_set(MenuSet),
            // Game
            GameSet.run_if(in_state(ApplicationState::Game)),
        ),
    );
}
