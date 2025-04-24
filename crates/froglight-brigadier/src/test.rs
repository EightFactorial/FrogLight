use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs::component::ComponentInfo;
use bevy_log::{Level, LogPlugin};
use froglight_common::entity::{EntityId, EntityUuid};
use uuid::Uuid;

use crate::{function::WorldRef, prelude::*};

#[test]
fn execute() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin { level: Level::INFO, ..default() }));
    app.add_plugins(BrigadierPlugin::default());

    // Add a basic command with no arguments
    app.add_command("test", |builder| {
        builder.command(|entity, _| {
            info!("Entity {entity}: Hello, world!");
        });
    });

    // Add a command with multiple arguments
    app.add_command("test_2", |builder| {
        let builder = builder.arg::<UVec2, _>().arg::<String, _>().arg::<String, _>();
        builder.command(|entity, vec2, string_1, string_2, _| {
            assert_eq!(vec2, [42, 42].into());
            info!("Entity {entity}: {string_1} {string_2}");
        });
    });

    // Add a command with a literal argument and showing how to access the `World`
    app.add_command("test_3", |builder| {
        let builder = builder.arg::<u32, _>().literal("literal").arg::<f64, _>();
        builder.command(|entity, int, double, mut world| {
            assert_eq!(int, 1000);
            assert_eq!(double.total_cmp(&40320.0), std::cmp::Ordering::Equal);

            let world = world.value();
            let components = world.inspect_entity(entity).unwrap().map(ComponentInfo::name);
            info!("Entity {entity}: {}", components.collect::<Vec<_>>().join(", "));
        });
    });

    // Add a command showing how to use forked commands.
    app.add_command("test_4", |builder| {
        builder.fork(|mut builder| {
            builder.branch().literal("first").command(|entity, _| {
                info!("Entity {entity}: First command");
            });
            builder.branch().literal("second").arg::<f32, _>().command(|entity, int, _| {
                info!("Entity {entity}: Second command: \"{int}\"");
            });
        });
    });

    // Add a command with optional arguments
    app.add_command("test_5", |builder| {
        fn function(entity: Entity, message: String, index: usize, _: WorldRef) {
            info!("Entity {entity}: Message \"{message}\", Index {index}");
            assert!(matches!(index, 0 | 10));
        }

        // Provide a default "Message" and "Index" value
        let builder = builder
            .command(|entity, world| function(entity, String::from("Default"), 0usize, world));

        // Provide a default "Index" value
        let builder = builder
            .arg::<String, _>()
            .command(|entity, string, world| function(entity, string, 0usize, world));

        // Parse all arguments
        builder.arg::<usize, _>().command(function);
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
        let bundle = (Name::new("TestEntity"), EntityId::from(0), EntityUuid::from(Uuid::nil()));
        let mut entity = commands.spawn(bundle);

        entity.run_command("test");
        entity.run_command("test_2 42 42 foo bar");
        entity.run_command("test_3 1000 literal 40320");

        entity.run_command("test_4 first");
        entity.run_command("test_4 second 3.14159");

        entity.run_command("test_5");
        entity.run_command("test_5 Provided");
        entity.run_command("test_5 Provided 10");

        entity.run_command("stop");
    });

    // Add a system to force the application to exit after some time
    #[expect(clippy::manual_assert)]
    app.add_systems(Update, |time: Res<Time>, mut timer: Local<Timer>| {
        if timer.elapsed().is_zero() {
            timer.set_duration(Duration::from_millis(100));
        }
        if timer.tick(time.elapsed()).just_finished() {
            panic!("Application did not exit after 100ms!");
        }
    });

    app.run()
}
