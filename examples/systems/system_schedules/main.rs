//! Example: "system_schedules"

use bevy::{app::AppExit, prelude::*};
use froglight::HeadlessPlugins;

fn main() {
    // Create a new App.
    let mut app = App::new();
    app.add_plugins(HeadlessPlugins);

    // Add a system to `PreStartup`.
    app.add_systems(PreStartup, pre_startup);

    // Add two systems to `Startup`, in a chain.
    app.add_systems(Startup, (startup_first, startup_second).chain());

    // Add a final system to `PostStartup`.
    app.add_systems(PostStartup, post_startup);

    // Add a system to close the application.
    app.add_systems(Update, close_app);

    app.run();
}

/// Run this function during [`PreStartup`].
fn pre_startup() {
    info!("This is sent during {PreStartup:?}.");
}

/// Run this function during [`Startup`].
fn startup_first() {
    info!("This is sent first during {Startup:?}.");
}

/// Run this function during [`Startup`].
fn startup_second() {
    info!("This is sent second during {Startup:?}.");
}

/// Run this function during [`PostStartup`].
fn post_startup() {
    info!("This is sent during {PostStartup:?}.");
}

/// Close the application.
fn close_app(mut send_exit: EventWriter<AppExit>) {
    info!("Closing the application!");
    send_exit.send(AppExit);
}
