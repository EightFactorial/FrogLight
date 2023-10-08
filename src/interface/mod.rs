use bevy::prelude::*;

pub mod menus;
pub mod player;

/// Add interface systems to the app
pub(super) fn setup(app: &mut App) {
    player::setup(app);
    menus::setup(app);
}
