use bevy::app::App;

pub mod resourcepacks;
pub mod textureatlases;

pub(super) fn setup(app: &mut App) {
    resourcepacks::setup(app);
    textureatlases::setup(app);
}
