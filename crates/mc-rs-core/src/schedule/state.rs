use bevy::prelude::*;

pub(super) fn configure(app: &mut App) { app.add_state::<ApplicationState>(); }

/// The current state of the application
///
/// This is used to determine which systems should be run
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum ApplicationState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}
