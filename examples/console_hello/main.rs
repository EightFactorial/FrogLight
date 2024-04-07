use bevy::{
    app::{App, Startup},
    log::info,
};
use froglight::AppPlugins;

/// The main function.
fn main() {
    // Create a new App.
    let mut app = App::new();

    // Add the `AppPlugins`.
    app.add_plugins(AppPlugins);

    // Run the function on startup.
    app.add_systems(Startup, my_cool_function);

    // Run the App.
    app.run();
}

/// My super cool function 😎
fn my_cool_function() {
    info!("Hello, world!");
}
