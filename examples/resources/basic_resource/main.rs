//! Example: "basic_resource"

use bevy::{app::AppExit, prelude::*};
use froglight::HeadlessPlugins;

fn main() {
    // Create a new App.
    let mut app = App::new();
    app.add_plugins(HeadlessPlugins);

    // Initialize the `Counter` resource.
    info!("Initializing a \"Counter\"!");
    app.init_resource::<Counter>();

    // Print the value of the `Counter` resource.
    app.add_systems(Startup, print_counter);

    // Close the application.
    app.add_systems(Update, close_app);

    app.run();
}

/// A simple counter resource.
///
/// This resource will be initialized with the value `0`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource)]
struct Counter(u32);

/// A system that prints the value of the counter.
fn print_counter(counter: Res<Counter>) {
    info!("Counter Value: \"{}\"", counter.0);
}

// A system that closes the application.
fn close_app(mut send_exit: EventWriter<AppExit>) {
    info!("Closing the application!");
    send_exit.send(AppExit);
}
