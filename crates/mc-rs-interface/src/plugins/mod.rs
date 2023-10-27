use bevy::prelude::App;
use mc_rs_render::RenderPlugin;

mod asset;
mod image;
mod window;

mod default;
use default::DefaultPlugin;

pub(super) fn setup(app: &mut App) { app.add_plugins((RenderPlugin, DefaultPlugin)); }
