use bevy::prelude::App;

pub mod player;
pub mod settings;

/// Adds all general systems to the app
pub(super) fn setup(app: &mut App) { player::add_systems(app); }
