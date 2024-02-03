use bevy::{asset::embedded_asset, prelude::*};
use froglight_client::plugins::{AppPlugins, LoadingPlugin};

mod plugin;
use plugin::GifLoadingPlugin;

/// Create a new application, but disable the default [`LoadingPlugin`] and use
/// our custom [`GifLoadingPlugin`] instead.
///
/// # Note
/// If you are importing [`froglight_client`] in your own project, you can skip
/// disabling the [`LoadingPlugin`] by disabling the `default-loading` feature.
fn main() {
    // Create a new bevy App
    let mut app = App::new();

    // Add the AppPlugins group, but disable the loading art from displaying
    app.add_plugins(AppPlugins::build(AppPlugins).set(LoadingPlugin::None));

    // Embed out GIF's split frames
    embedded_asset!(app, "my_gif_frames.png");

    // Add our custom GifLoadingPlugin
    app.add_plugins(GifLoadingPlugin {
        path: "embedded://loading_gif/my_gif_frames.png".to_string(),
        frame_height: 100.0,
        frame_width: 100.0,
        frame_count: 10,
    });

    // Run the app!
    app.run();
}
