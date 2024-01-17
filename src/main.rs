use bevy::app::App;
// use froglight_client::FrogLightAppPlugins;

/// The global allocator.
///
/// This is completely optional, but might improve performance.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    // Create a new application.
    let mut app = App::new();

    // // Add both the FrogLight plugins and the Bevy plugins.
    // app.add_plugins(FrogLightAppPlugins);

    // Run the application.
    app.run();
}
