use bevy::prelude::App;

pub mod assets;
pub mod menus;

pub(super) fn setup(app: &mut App) {
    menus::setup(app);
    assets::setup(app);
}
