use bevy::prelude::App;

pub mod blocks;
pub mod states;
pub mod world;

/// Adds all general systems to the app
pub(super) fn setup(app: &mut App) { states::add_states(app); }
