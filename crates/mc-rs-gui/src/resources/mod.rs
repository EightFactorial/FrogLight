use bevy::prelude::*;

pub mod camera;
pub mod font;
pub mod scale;
pub mod servers;

pub(super) fn setup(app: &mut App) {
    scale::setup(app);
    font::setup(app);
}
