use bevy::prelude::App;

pub mod block;
pub mod block_list;

/// Adds all block systems to the app
pub(super) fn add_systems(app: &mut App) { block_list::add_systems(app); }
