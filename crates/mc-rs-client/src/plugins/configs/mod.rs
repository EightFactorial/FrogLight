//! Configuration file plugin

use bevy::{input::mouse::MouseMotion, prelude::*};

pub mod keybinds;
use keybinds::Keybinds;

pub mod servers;
use mc_rs_core::{
    components::player::{ControlledPlayer, ControlledPlayerHead},
    resources::client_information::ClientInformation,
};
use mc_rs_gui::{menus::states::menus::MenuComponentState, resources::servers::ServerList};
use servers::SettingsServerList;

pub mod settings;
use settings::Settings;

pub(crate) mod traits;
use traits::{ConfigFile, ResourceConfig};

/// A plugin thats loads all of the config files to the app.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        // Add the keybinds to the app
        Keybinds::add_systems(app);
        app.insert_resource(Keybinds::load());

        // Add the server list to the app
        SettingsServerList::add_systems(app);
        app.insert_resource(ServerList::from(SettingsServerList::load()));

        // Add the settings to the app
        Settings::add_systems(app);
        let settings = Settings::load();
        settings.insert_resources(app);
        app.insert_resource(settings);

        // Add the client config to the app
        app.init_resource::<ClientInformation>();

        app.add_systems(
            Update,
            (move_camera, move_player).run_if(in_state(MenuComponentState::InGame)),
        );
    }
}

fn move_player(
    mut query: Query<&mut Transform, With<ControlledPlayer>>,
    input_keyboard: Res<Input<KeyCode>>,
    input_mouse: Res<Input<MouseButton>>,

    keybinds: Res<Keybinds>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut v = Vec3::ZERO;

        if let Some(bind) = keybinds.movement.forward {
            if bind.pressed(&input_keyboard, &input_mouse) {
                v -= transform.local_z() * 0.1;
            }
        }
        if let Some(bind) = keybinds.movement.backward {
            if bind.pressed(&input_keyboard, &input_mouse) {
                v += transform.local_z() * 0.1;
            }
        }
        if let Some(bind) = keybinds.movement.left {
            if bind.pressed(&input_keyboard, &input_mouse) {
                v -= transform.local_x() * 0.1;
            }
        }
        if let Some(bind) = keybinds.movement.right {
            if bind.pressed(&input_keyboard, &input_mouse) {
                v += transform.local_x() * 0.1;
            }
        }

        if let Some(bind) = keybinds.movement.jump {
            if bind.pressed(&input_keyboard, &input_mouse) {
                v += Vec3::Y * 0.1;
            }
        }
        if let Some(bind) = keybinds.movement.sneak {
            if bind.pressed(&input_keyboard, &input_mouse) {
                v -= Vec3::Y * 0.1;
            }
        }

        if v == Vec3::ZERO {
            return;
        }

        v = v.normalize();

        if let Some(bind) = keybinds.movement.sprint {
            if bind.pressed(&input_keyboard, &input_mouse) {
                v *= 10.0;
            }
        }

        transform.translation += v;
    } else {
        #[cfg(any(debug_assertions, feature = "debug"))]
        error!("Failed to get ControlledPlayer transform");
    }
}

fn move_camera(
    mut query: Query<&mut Transform, With<ControlledPlayerHead>>,
    mut events: EventReader<MouseMotion>,
) {
    let mut cumulative = Vec2::ZERO;
    for event in events.read() {
        cumulative += event.delta;
    }

    if let Ok(mut transform) = query.get_single_mut() {
        transform.rotate_x(cumulative.y * 0.001);
        transform.rotate_local_y(cumulative.x * 0.001);
    } else {
        #[cfg(any(debug_assertions, feature = "debug"))]
        error!("Failed to get ControlledPlayerHead transform");
    }
}
