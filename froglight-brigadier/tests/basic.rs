//! TODO
#![no_std]

use bevy::{log::LogPlugin, prelude::*};
use froglight_brigadier::{bevy::BrigadierPlugin, prelude::*};

#[test]
fn basic() -> AppExit {
    let mut app = App::new();
    app.add_plugins((LogPlugin::default(), MinimalPlugins));
    app.add_plugins(BrigadierPlugin);

    app.add_game_command::<(), _>("test_1", |_ctx: In<CommandCtx<()>>| {});
    app.add_game_command::<u32, _>("test_2", |_ctx: In<CommandCtx<u32>>| {});
    // app.add_game_command::<StringWord, _>("test_3", |_ctx:
    // In<CommandCtx<StringWord>>| {});

    // Exit after 0.25 seconds
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        if time.elapsed_secs() > 0.25 {
            commands.write_message(AppExit::Success);
        }
    });

    app.run()
}
