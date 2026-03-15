//! TODO
#![no_std]

use bevy::{log::LogPlugin, prelude::*};
// use froglight_brigadier::{bevy::BrigadierPlugin, parse::string::StringWord,
// prelude::*};

#[test]
fn basic() -> AppExit {
    let mut app = App::new();
    app.add_plugins((LogPlugin::default(), MinimalPlugins));
    // app.add_plugins(BrigadierPlugin);

    // Exit after 0.25 seconds
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        if time.elapsed_secs() > 0.25 {
            commands.write_message(AppExit::Success);
        }
    });

    app.run()
}
