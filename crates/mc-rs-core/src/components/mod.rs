use bevy::prelude::App;

pub mod player;

pub(super) fn setup(app: &mut App) { player::setup(app); }
