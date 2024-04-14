//! Example: "custom_plugin"

use bevy::{app::AppExit, prelude::*};
use froglight::HeadlessPlugins;

fn main() {
    // Create a new App.
    let mut app = App::new();
    app.add_plugins(HeadlessPlugins);

    // Add the custom plugin.
    app.add_plugins(MyCustomPlugin);

    app.run();
}

/// A custom bevy [`Plugin`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct MyCustomPlugin;

impl Plugin for MyCustomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::print_hello);
        app.add_systems(Update, Self::close_app);
    }
}

impl MyCustomPlugin {
    /// A simple system that prints a message.
    fn print_hello() {
        info!("Hello from MyCustomPlugin!");
    }

    /// A system that closes the application.
    fn close_app(mut send_exit: EventWriter<AppExit>) {
        info!("Closing the application!");
        send_exit.send(AppExit);
    }
}
