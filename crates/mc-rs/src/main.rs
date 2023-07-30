use bevy::app::App;

mod plugins;

fn main() {
    // Create a new app
    let mut app = App::new();

    // Add plugins
    plugins::add_plugins(&mut app);

    // Run the app
    app.run();
}
