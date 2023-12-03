use bevy::app::App;

pub mod block_background;

pub(super) fn setup(app: &mut App) { block_background::setup(app); }
