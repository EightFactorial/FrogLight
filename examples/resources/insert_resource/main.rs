//! Example: "insert_resource"

use bevy::{app::AppExit, prelude::*};
use froglight::HeadlessPlugins;

fn main() {
    // Create a new App.
    let mut app = App::new();
    app.add_plugins(HeadlessPlugins);

    // Initialize the `InitCounter` resource.
    info!("Initializing an \"InitCounter\"!");
    app.init_resource::<InitCounter>();

    // Insert the `InsertCounter` resource.
    info!("Inserting an \"InsertCounter\"!");
    app.insert_resource(InsertCounter::default());

    // Add a system to print the values of the counters.
    app.add_systems(Startup, print_counters);

    // Close the application.
    app.add_systems(Update, close_app);

    app.run();
}

/// A simple counter resource.
///
/// This resource will be initialized with the value `0`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource)]
struct InitCounter(u32);

/// A simple counter resource.
///
/// This resource will be initialized with the value `10`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Resource)]
struct InsertCounter(u32);

impl Default for InsertCounter {
    fn default() -> Self { Self(10) }
}

/// A system that prints the values of the counters.
fn print_counters(init_counter: Res<InitCounter>, insert_counter: Res<InsertCounter>) {
    info!("InitCounter Value: \"{}\"", init_counter.0);
    info!("InsertCounter Value: \"{}\"", insert_counter.0);
}

// A system that closes the application.
fn close_app(mut send_exit: EventWriter<AppExit>) {
    info!("Closing the application!");
    send_exit.send(AppExit);
}
