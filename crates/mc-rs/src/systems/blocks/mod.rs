use bevy::prelude::App;

pub mod attributes;
pub mod block;
pub mod list;

/// Adds all block systems to the app
pub(super) fn add_systems(app: &mut App) { list::add_systems(app); }
