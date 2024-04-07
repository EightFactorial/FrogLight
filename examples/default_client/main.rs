use bevy::app::App;
use froglight::AppPlugins;

/// The main function.
///
/// This is the exact same as the default [`froglight`] client!
fn main() {
    // Create a new App.
    let mut app = App::new();

    // Add the `AppPlugins`.
    app.add_plugins(AppPlugins);

    // Run the App.
    app.run();
}
