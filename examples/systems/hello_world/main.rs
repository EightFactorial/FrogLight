use bevy::{app::AppExit, prelude::*};
use froglight::HeadlessPlugins;

fn main() {
    // Create a new App.
    let mut app = App::new();
    app.add_plugins(HeadlessPlugins);

    // Run the function on `Startup`.
    //
    // This will only run once, when the App is started.
    app.add_systems(Startup, my_cool_function);

    // Close the application.
    //
    // This will run every frame, but because
    // it closes the application, it will only run once.
    app.add_systems(Update, close_app);

    app.run();
}

/// Damn I'm good 😎
fn my_cool_function() {
    info!("Hello, world!");
}

fn close_app(mut send_exit: EventWriter<AppExit>) {
    info!("Closing the application!");
    send_exit.send(AppExit);
}
