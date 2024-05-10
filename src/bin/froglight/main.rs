#![doc = include_str!("README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::prelude::*;
use froglight::AppPlugins;

/// The global allocator.
///
/// This is completely optional, but might improve performance.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// The main function.
///
/// Create a new [`App`], add the [`AppPlugins`], and run it.
fn main() {
    let mut app = App::new();
    app.add_plugins(AppPlugins);

    // Add the `WorldInspectorPlugin` if the `inspector` feature is enabled.
    #[cfg(feature = "inspector")]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app.add_plugins(
            WorldInspectorPlugin::new()
                // Toggle `state` if holding `KeyCode::F3` and pressing `KeyCode::I`.
                .run_if(|input: Res<ButtonInput<KeyCode>>, mut state: Local<bool>| {
                    if input.pressed(KeyCode::F3) && input.just_pressed(KeyCode::KeyI) {
                        *state = !*state;
                    }
                    *state
                }),
        );
    }

    app.run();
}
