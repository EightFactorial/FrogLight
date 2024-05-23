use bevy::{app::Plugin, prelude::*};
use bevy_inspector_egui::{inspector_egui_impls::InspectorEguiImpl, quick::WorldInspectorPlugin};
use froglight_components::resourcekey::ResourceKey;

/// A plugin for enabling the [`WorldInspectorPlugin`].
///
/// Additionally adds a keybind to show/hide the inspector.
///
/// This is very useful for debugging and inspecting entities,
/// but should not be enabled for release builds.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InspectorPlugin;

impl InspectorPlugin {
    /// The key code to hold for the inspector.
    const HOLD_KEY: KeyCode = KeyCode::F3;
    /// The key code to press for the inspector.
    const PRESS_KEY: KeyCode = KeyCode::KeyI;
}

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        // Log that the `WorldInspector` is enabled.
        info!("World Inspector enabled, press {:?} + {:?}", Self::HOLD_KEY, Self::PRESS_KEY);

        // Add the `WorldInspectorPlugin` with a toggle condition.
        app.add_plugins(WorldInspectorPlugin::new().run_if(
            |input: Res<ButtonInput<KeyCode>>, mut state: Local<bool>| {
                // Toggle the enable state with F3 + I.
                if input.just_pressed(Self::PRESS_KEY) && input.pressed(Self::HOLD_KEY) {
                    *state = !*state;
                }
                *state
            },
        ));

        // Register the `ResourceKey` type early
        // so it doesn't panic when registering type data.
        app.register_type::<ResourceKey>();
        // Register the `ResourceKey` type data.
        app.register_type_data::<ResourceKey, InspectorEguiImpl>();
    }
}
