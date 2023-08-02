use bevy::prelude::App;

pub mod blocks;
pub mod player;
pub mod settings;
pub mod states;
pub mod world;

/// Adds all general systems to the app
pub(super) fn setup(app: &mut App) {
    states::add_states(app);
    blocks::add_systems(app);
    player::add_systems(app);
}
