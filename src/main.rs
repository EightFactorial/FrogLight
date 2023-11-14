use bevy::app::App;

use mc_rs_interface::InterfacePlugin;
use mc_rs_network::NetworkingPlugin;

fn main() {
    // Create a new app
    let mut app = App::new();

    // Add the plugins
    app.add_plugins((InterfacePlugin, NetworkingPlugin));

    // Run the app
    app.run();
}

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
