use bevy::prelude::App;

pub mod app_state;
pub mod blocks;
pub mod player;
pub mod settings;
pub mod world;

/// Adds all general systems to the app
pub(super) fn setup(app: &mut App) {
    app_state::configure(app);
    blocks::add_systems(app);
    player::add_systems(app);
    world::add_systems(app);
}
