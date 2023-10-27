use bevy::prelude::*;

pub mod task;

/// Adds the `Worlds` resource and its systems.
pub(super) fn setup(app: &mut App) { task::setup(app); }
