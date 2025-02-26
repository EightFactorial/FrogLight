use bevy::prelude::*;
use bevy_ecs::component::ComponentInfo;
use bevy_log::{Level, LogPlugin};
use froglight_common::{EntityId, EntityUuid};
use uuid::Uuid;

use crate::prelude::*;

#[test]
fn execute() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin { level: Level::INFO, ..default() }));
    app.add_plugins(BrigadierPlugin::default());

    // Add a basic command with no arguments
    app.add_command("test", |builder| {
        builder.command(|entity, _world| {
            info!("Entity {entity}: Hello, world!");
        });
    });

    // Add a command with multiple arguments
    app.add_command("test_2", |builder| {
        let builder = builder.arg::<u32, _>().arg::<f64, _>().arg::<String, _>().arg::<String, _>();
        builder.command(|entity, num, float, string_1, string_2, _world| {
            assert_eq!(num, 42);
            assert_eq!(float.total_cmp(&42.0), std::cmp::Ordering::Equal);
            info!("Entity {entity}: {string_1} {string_2}");
        });
    });

    // Add a command with a literal argument and showing how to access the `World`
    app.add_command("test_3", |builder| {
        let builder = builder.arg::<u32, _>().literal("literal").arg::<f64, _>();
        builder.command(|entity, num, float, mut world| {
            assert_eq!(num, 1000);
            assert_eq!(float.total_cmp(&40320.0), std::cmp::Ordering::Equal);

            let world = world.value();
            let components = world.inspect_entity(entity).map(ComponentInfo::name);
            info!("Entity {entity}: {}", components.collect::<Vec<_>>().join(", "));
        });
    });

    // Add a command to stop the application and exit
    app.add_command("stop", |builder| {
        builder.command(|entity, mut world| {
            info!("Entity {entity}: Stopping application...");
            world.value().send_event(AppExit::default());
        });
    });

    // Add a system spawn an entity and run the commands
    app.add_systems(Update, |mut commands: Commands| {
        let mut entity = commands.spawn((EntityId::from(0), EntityUuid::from(Uuid::nil())));
        entity.run_command("test");
        entity.run_command("test_2 42 42.0 foo bar");
        entity.run_command("test_3 1000 literal 40320.0");
        entity.run_command("stop");
    });

    app.run()
}
