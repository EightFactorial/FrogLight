//! Example: "disable_logs"

use bevy::{app::AppExit, log::LogPlugin, prelude::*};
use froglight::HeadlessPlugins;

fn main() {
    // Create a new App.
    let mut app = App::new();

    // Disable bevy's `LogPlugin`
    {
        // Get the `HeadlessPlugins` PluginGroup
        let mut headless = HeadlessPlugins.build();

        // Disable the `LogPlugin`
        headless = headless.disable::<LogPlugin>();

        // Add the modified plugin group to the App
        app.add_plugins(headless);
    }

    // Add systems to the App
    app.add_systems(Startup, print_hello);
    app.add_systems(Update, close_app);

    // Run the App.
    app.run();
}

/// A system that prints a message.
fn print_hello() {
    info!("Huh, where did the logs go?");
}

/// A system that closes the application.
fn close_app(mut send_exit: EventWriter<AppExit>) {
    info!("Closing the application!");
    send_exit.send(AppExit);
}
