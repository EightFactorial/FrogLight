#![doc = include_str!("README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::prelude::*;
use froglight::AppPlugins;

#[cfg(any(target_os = "windows", target_os = "linux"))]
mod window_icon;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod window_title;

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

    // Add the `WindowIconPlugin` if the target OS is Windows or Linux.
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        app.add_plugins(window_icon::WindowIconPlugin);
    }

    // Add the `WindowTitlePlugin` if the target OS is not Android or iOS.
    #[cfg(all(not(target_os = "android"), not(target_os = "ios")))]
    {
        app.add_plugins(window_title::WindowTitlePlugin);
    }

    // Add the `WorldInspectorPlugin` if the `inspector` feature is enabled.
    #[cfg(feature = "inspector")]
    {
        info!("World Inspector enabled, press F3 + I");
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new().run_if(
            |input: Res<ButtonInput<KeyCode>>, mut state: Local<bool>| {
                if input.pressed(KeyCode::F3) && input.just_pressed(KeyCode::KeyI) {
                    *state = !*state;
                }
                *state
            },
        ));
    }

    app.run();
}
