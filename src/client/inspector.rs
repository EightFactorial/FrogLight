use bevy::{
    app::{App, Plugin},
    ecs::system::{Local, Res},
    input::{keyboard::KeyCode, ButtonInput},
    log::info,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        info!("Use F3 + I to toggle the inspector!");
        app.add_plugins(WorldInspectorPlugin::new().run_if(Self::input_toggle));
    }
}

impl InspectorPlugin {
    fn input_toggle(input: Res<ButtonInput<KeyCode>>, mut state: Local<bool>) -> bool {
        if input.just_pressed(KeyCode::KeyI) && input.pressed(KeyCode::F3) {
            *state = !*state;
        }
        *state
    }
}
