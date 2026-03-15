//! TODO
#![no_std]

use bevy::{log::LogPlugin, prelude::*};
use froglight_brigadier::{bevy::BrigadierPlugin, parse::string::StringWord, prelude::*};

#[test]
fn basic() -> AppExit {
    let mut app = App::new();
    app.add_plugins((LogPlugin::default(), MinimalPlugins));
    app.add_plugins(BrigadierPlugin);

    // Test using `with` and `build` separately.
    app.add_game_command("test_1", |builder| {
        let mut builder = builder.with::<i32>();
        builder.build(|_int| todo!());
        builder.with::<StringWord>().build(|_int, _word| todo!());
    });

    // Test using `build_and` to chain arguments together.
    app.add_game_command("test_2", |builder| {
        let builder = builder.build_and::<StringWord, _, _>(|| todo!());
        builder.build_and::<u32, _, _>(|_word| todo!()).build(|_word, _int| todo!());
    });

    // Test using `with_bundle` to add multiple arguments at once.
    app.add_game_command("test_3", |builder| {
        let mut builder = builder.with_bundle::<(i32, i32, i32)>();
        builder.build(|_x, _y, _z| todo!());
    });

    // Exit after 0.25 seconds
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        if time.elapsed_secs() > 0.25 {
            commands.write_message(AppExit::Success);
        }
    });

    app.run()
}
