//! TODO
#![no_std]

use bevy::{log::LogPlugin, prelude::*};
use froglight_brigadier::{argument::StringType, bevy::BrigadierPlugin, prelude::*};

#[test]
fn basic() -> AppExit {
    let mut app = App::new();
    app.add_plugins((LogPlugin::default(), MinimalPlugins));
    app.add_plugins(BrigadierPlugin);

    app.add_game_command("test_1", |_ctx: In<CommandCtx<()>>| {});
    app.add_game_command("test_2", |_ctx: In<CommandCtx<u32>>| {});
    app.add_game_command("test_3", |_ctx: In<CommandCtx<(i64, i64)>>| {});

    app.add_game_command("test_string_default", |_ctx: In<CommandCtx<String>>| {});
    app.add_game_command_using(
        "test_string_word",
        StringType::Word,
        |_ctx: In<CommandCtx<String>>| {},
    );
    app.add_game_command_using(
        "test_string_greedy",
        StringType::Greedy,
        |_ctx: In<CommandCtx<String>>| {},
    );

    // Exit after 0.25 seconds
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        if time.elapsed_secs() > 0.25 {
            commands.write_message(AppExit::Success);
        }
    });

    app.run()
}
