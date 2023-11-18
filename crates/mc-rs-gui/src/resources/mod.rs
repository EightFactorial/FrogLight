use bevy::prelude::*;

pub mod camera;
pub mod gui;

pub(super) fn setup(app: &mut App) { gui::setup(app); }
