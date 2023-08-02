use bevy::prelude::*;

/// The current state of the application
///
/// This is used to determine which systems should be run
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum ApplicationState {
    #[cfg(feature = "splash")]
    #[cfg_attr(feature = "splash", default)]
    SplashScreen,
    #[cfg_attr(not(feature = "splash"), default)]
    MainMenu,
    InGame,
    Paused,
}

/// A system set that runs when the [ApplicationState] is either
/// [MainMenu](ApplicationState::MainMenu) or [SplashScreen](ApplicationState::SplashScreen)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MenuSet;

/// A system set that runs when the [ApplicationState] is [MainMenu](ApplicationState::MainMenu)
/// state
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MainMenuSet;

/// A system set that runs when the [ApplicationState] is either
/// [InGame](ApplicationState::InGame) or [Paused](ApplicationState::Paused)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct GameSet;

/// A system set that runs when the [ApplicationState] is [InGame](ApplicationState::InGame)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct InGameSet;

/// A system set that runs when the [ApplicationState] is [Paused](ApplicationState::Paused)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct PausedSet;

/// Adds the application state to the app
pub(super) fn add_state(app: &mut App) {
    app.add_state::<ApplicationState>();

    app.configure_sets(
        Update,
        (
            // Splash and MainMenu
            #[cfg(feature = "splash")]
            {
                MenuSet.run_if(
                    in_state(ApplicationState::MainMenu)
                        .or_else(in_state(ApplicationState::SplashScreen)),
                )
            },
            #[cfg(not(feature = "splash"))]
            {
                MenuSet.run_if(in_state(ApplicationState::MainMenu))
            },
            MainMenuSet
                .run_if(in_state(ApplicationState::MainMenu))
                .ambiguous_with(MenuSet),
            // InGame and Paused
            GameSet.run_if(
                in_state(ApplicationState::InGame).or_else(in_state(ApplicationState::Paused)),
            ),
            InGameSet
                .run_if(in_state(ApplicationState::InGame))
                .ambiguous_with(GameSet),
            PausedSet
                .run_if(in_state(ApplicationState::Paused))
                .ambiguous_with(GameSet),
        ),
    );
}
