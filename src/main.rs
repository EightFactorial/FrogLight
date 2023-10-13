#![feature(future_join)]
#![feature(trivial_bounds)]

use bevy::app::App;

mod interface;
mod networking;
mod plugins;
mod systems;
mod util;

fn main() {
    let fail_clippy = 1;
    let fail_clippy_2 = 2;

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

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
