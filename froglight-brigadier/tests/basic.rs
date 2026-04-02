//! TODO
#![no_std]

use bevy::{log::LogPlugin, prelude::*};
use froglight_brigadier::{argument::StringType, bevy::BrigadierPlugin, prelude::*};

#[test]
fn basic() -> AppExit {
    let mut app = App::new();
    app.add_plugins((LogPlugin::default(), MinimalPlugins));
    app.add_plugins(BrigadierPlugin);

    app.add_game_command("test_1", |_ctx: GameCommandCtx<()>| {});
    app.add_game_command("test_2", |_ctx: GameCommandCtx<u32>| {});
    app.add_game_command("test_3", |_ctx: GameCommandCtx<(i64, i64)>| {});

    app.add_game_command("test_string_a", |_ctx: GameCommandCtx<String>| {});
    app.add_game_command_using(
        "test_string_b",
        StringType::Word,
        |_ctx: GameCommandCtx<String>| {},
    );
    app.add_game_command_using(
        "test_string_c",
        StringType::Greedy,
        |_ctx: GameCommandCtx<String>| {},
    );

    app.add_game_command("exit", |_: GameCommandCtx<()>, mut commands: Commands| {
        commands.write_message(AppExit::Success);
    });

    // Exit after 0.20 seconds
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        if time.elapsed_secs() > 0.20 {
            #[cfg(debug_assertions)]
            info!("Exiting via command...");
            commands.spawn_empty().game_command("exit");
        }
    });

    // Fail after 0.25 seconds
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        if time.elapsed_secs() > 0.25 {
            #[cfg(debug_assertions)]
            error!("Exiting via timeout...");
            commands.write_message(AppExit::error());
        }
    });

    app.run()
}
