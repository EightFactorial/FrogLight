use bevy::prelude::App;

pub mod username;

pub(super) fn setup(app: &mut App) { username::setup(app); }
