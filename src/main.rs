use bevy::app::App;
use mc_rs_client::ClientPlugins;

fn main() { App::new().add_plugins(ClientPlugins).run(); }

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
