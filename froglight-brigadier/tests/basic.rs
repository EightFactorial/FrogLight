//! TODO
#![no_std]

use bevy::{log::LogPlugin, prelude::*};
use froglight_brigadier::{argument::StringType, bevy::BrigadierPlugin, prelude::*};

#[test]
fn basic() -> AppExit {
    let mut app = App::new();
    app.add_plugins((LogPlugin::default(), MinimalPlugins));
    app.add_plugins(BrigadierPlugin);

    app.add_game_command("test_1", |ctx: GameCommandCtx<()>| {
        info!("Executed \"test_1\": {:?}", ctx.into_input());
    });
    app.add_game_command("test_2", |ctx: GameCommandCtx<u32>| {
        info!("Executed \"test_2\": {:?}", ctx.into_input());
    });
    app.add_game_command("test_3", |ctx: GameCommandCtx<(i64, i64)>| {
        info!("Executed \"test_3\": {:?}", ctx.into_input());
    });

    app.add_game_command("test_string_a", |ctx: GameCommandCtx<String>| {
        info!("Executed \"test_string_a\": {:?}", ctx.into_input());
    });
    app.add_game_command_using("test_string_b", StringType::Word, |ctx: GameCommandCtx<String>| {
        info!("Executed \"test_string_b\": {:?}", ctx.into_input());
    });
    app.add_game_command_using(
        "test_string_c",
        StringType::Greedy,
        |ctx: GameCommandCtx<String>| {
            info!("Executed \"test_string_c\": {:?}", ctx.into_input());
        },
    );

    #[cfg(feature = "glam")]
    app.add_game_command("test_glam_vec3", |ctx: GameCommandCtx<glam::Vec3>| {
        info!("Executed \"test_glam_vec3\": {:?}", ctx.into_input());
    });
    #[cfg(feature = "glam")]
    app.add_game_command("test_glam_bvec2", |ctx: GameCommandCtx<glam::BVec2>| {
        info!("Executed \"test_glam_bvec2\": {:?}", ctx.into_input());
    });

    app.add_game_command("exit", |_: GameCommandCtx<()>, mut commands: Commands| {
        commands.write_message(AppExit::Success);
    });

    app.add_systems(Startup, |mut commands: Commands| {
        let mut entity = commands.spawn_empty();

        entity.game_command("test_1");
        entity.game_command("test_2 32");
        entity.game_command("test_3 64 128");

        entity.game_command("test_string_a Hello World!"); // Expected to ERROR
        entity.game_command("test_string_b Hello World!"); // Expected to ERROR
        entity.game_command("test_string_c Hello World!");

        #[cfg(feature = "glam")]
        entity.game_command("test_glam_vec3 1.0 2.0 3.0");
        #[cfg(feature = "glam")]
        entity.game_command("test_glam_bvec2 false true");

        entity.game_command("exit");
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
