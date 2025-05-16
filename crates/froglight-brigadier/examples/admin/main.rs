//! TODO

use std::time::Duration;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use froglight_brigadier::{plugin::BrigadierPlugin, prelude::*};

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, BrigadierPlugin::default()));
    app.add_plugins(LogPlugin { level: Level::INFO, ..default() });
    app.add_systems(Update, timeout);

    // Add a command to quit the application
    app.add_command("quit", |builder| {
        builder.command(|entity: Entity, mut world: WorldRef| {
            info!("Entity {entity}: Stopping the application!");
            world.value().send_event(AppExit::Success);
        });
    });

    app.run()
}

/// A system that forces the application to exit after one second.
fn timeout(time: Res<Time>, mut timer: Local<Timer>, mut commands: Commands) {
    if timer.elapsed().is_zero() {
        timer.set_duration(Duration::from_secs(1));
    }
    if timer.tick(time.delta()).just_finished() {
        error!("Timeout reached, exiting application!");
        commands.send_event(AppExit::error());
    }
}
