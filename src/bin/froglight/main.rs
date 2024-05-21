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

    // Print the MiMalloc version if the `mimalloc` feature is enabled.
    #[cfg(feature = "mimalloc")]
    {
        info!("Using MiMalloc v{}", GLOBAL.version());
    }

    // Add the `WindowIconPlugin` if the target OS is Windows or Linux.
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        app.add_plugins(window_icon::WindowIconPlugin);
    }

    // Add the `WindowTitlePlugin` if the target OS is not Android or iOS.
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        app.add_plugins(window_title::WindowTitlePlugin);
    }

    // Add the `WorldInspectorPlugin` if the `inspector` feature is enabled.
    #[cfg(feature = "inspector")]
    {
        info!("World Inspector enabled, press F3 + I");
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new().run_if(
            |input: Res<ButtonInput<KeyCode>>, mut state: Local<bool>| {
                // Toggle the enable state with F3 + I.
                if input.just_pressed(KeyCode::KeyI) && input.pressed(KeyCode::F3) {
                    *state = !*state;
                }
                *state
            },
        ));

        // Register the `ResourceKey` type.
        app.register_type::<froglight::network::common::ResourceKey>();
        // Register the `ResourceKey` type data.
        app.register_type_data::<froglight::network::common::ResourceKey, bevy_inspector_egui::inspector_egui_impls::InspectorEguiImpl>();
    }

    app.run();
}
