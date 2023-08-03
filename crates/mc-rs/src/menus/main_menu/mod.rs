use bevy::prelude::App;

pub mod backgrounds;

/// Set up the main menu
pub(super) fn setup_menu(app: &mut App) { backgrounds::setup_backgrounds(app); }
