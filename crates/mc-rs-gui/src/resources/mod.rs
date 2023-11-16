use bevy::prelude::*;

pub mod gui;

pub(super) fn setup(app: &mut App) { gui::setup(app); }
