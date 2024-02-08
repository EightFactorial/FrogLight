use std::time::Duration;

use bevy::{asset::embedded_asset, prelude::*};
use froglight_client::plugins::{AppPlugins, LoadingPlugin};

mod plugin;
use plugin::GifLoadingPlugin;

/// Create a new application, but set the [`LoadingPlugin`] to
/// [`LoadingPlugin::None`].
///
/// This prevents any loading art from displaying, and allows us to set up
/// our own custom art and animations.
fn main() {
    // Create a new bevy App
    let mut app = App::new();

    // Add the AppPlugins group, but disable the loading art from displaying
    app.add_plugins(AppPlugins::build(AppPlugins).set(LoadingPlugin::None));

    // Embed out GIF's split frames
    embedded_asset!(app, "", "maxwell_split_frames.png");

    // Add our custom GifLoadingPlugin
    app.add_plugins(GifLoadingPlugin {
        duration: Duration::from_secs_f32(1.0 / 30.0),
        path: "embedded://loading_gif/maxwell_split_frames.png".to_string(),
        frame_dimensions: Vec2::new(360.0, 241.0),
        frame_tiling: UVec2::new(2, 94),
        frame_count: 187,
    });

    // Run the app!
    app.run();
}
