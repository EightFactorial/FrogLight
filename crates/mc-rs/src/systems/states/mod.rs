use bevy::prelude::App;

pub mod application;
pub mod menu;

/// Adds all states to the app
pub(super) fn add_states(app: &mut App) {
    application::add_state(app);
    menu::add_state(app);
}
