#![feature(trivial_bounds)]

use bevy::app::App;
use mc_rs_core::CorePlugin;
use mc_rs_network::NetworkingPlugin;

mod interface;
mod plugins;
mod systems;
mod util;

fn main() {
    // Create a new app
    let mut app = App::new();

    // Add plugins
    plugins::add_plugins(&mut app);

    // // Add networking systems
    // networking::setup(&mut app);

    app.add_plugins((CorePlugin, NetworkingPlugin));

    // Add interface systems
    interface::setup(&mut app);

    // Add general systems
    systems::setup(&mut app);

    // Run the app
    app.run();
}

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
