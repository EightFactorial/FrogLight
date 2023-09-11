#![feature(future_join)]
#![feature(trivial_bounds)]

use bevy::app::App;

mod interface;
mod networking;
mod plugins;
mod systems;
mod util;

fn main() {
    // Create a new app
    let mut app = App::new();

    // Add plugins
    plugins::add_plugins(&mut app);

    // Add networking systems
    networking::setup(&mut app);

    // Add interface systems
    interface::setup(&mut app);

    // Add general systems
    systems::setup(&mut app);

    // Run the app
    app.run();
}
