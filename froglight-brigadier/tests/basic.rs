//! TODO
#![no_std]

use bevy::{log::LogPlugin, prelude::*};
use froglight_brigadier::{argument::StringType, bevy::BrigadierPlugin, prelude::*};

#[test]
fn basic() -> AppExit {
    let mut app = App::new();
    app.add_plugins((LogPlugin::default(), MinimalPlugins));
    app.add_plugins(BrigadierPlugin);

    app.add_game_command("test_1", |_ctx: CommandCtx<()>| {});
    app.add_game_command("test_2", |_ctx: CommandCtx<u32>| {});
    app.add_game_command("test_3", |_ctx: CommandCtx<(i64, i64)>| {});

    app.add_game_command("test_string_a", |_ctx: CommandCtx<String>| {});
    app.add_game_command_using("test_string_b", StringType::Word, |_ctx: CommandCtx<String>| {});
    app.add_game_command_using("test_string_c", StringType::Greedy, |_ctx: CommandCtx<String>| {});

    // Exit after 0.25 seconds
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        if time.elapsed_secs() > 0.25 {
            commands.write_message(AppExit::Success);
        }
    });

    app.run()
}
