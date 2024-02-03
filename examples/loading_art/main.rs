use bevy::{asset::embedded_asset, prelude::*};
use froglight_client::plugins::{AppPlugins, LoadingPlugin};

/// Create a new application, but set the
/// loading plugin to use a different image.
fn main() {
    // Create a new bevy App
    let mut app = App::new();

    // Create the AppPlugins group
    let mut plugins = AppPlugins::build(AppPlugins);

    // Set the loading screen to use the embedded image
    // Because this is an example, the path is different than the README.md
    plugins = plugins.set(LoadingPlugin::new("embedded://loading_art/my_custom_art.png"));

    app.add_plugins(plugins);

    // Place the image next to the main.rs file
    //
    // Because this is an example without a `src` dir,
    // the path is different than the README.md
    embedded_asset!(app, "", "my_custom_art.png");

    // Run the application!
    app.run();
}
