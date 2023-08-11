use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{
    networking::network::{LocalPlayer, LocalPlayerComponent, LocalPlayerHead},
    systems::app_state::GameSet,
};

use super::resources::Paused;

pub(super) fn add_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_keyboard.run_if(any_keyboard_events),
            handle_mouse.run_if(any_mouse_events),
        )
            .run_if(resource_exists_and_equals(Paused(false)))
            .in_set(GameSet),
    );
}

fn any_keyboard_events(keyboard: Res<Input<KeyCode>>) -> bool {
    keyboard.any_pressed([
        KeyCode::W,
        KeyCode::A,
        KeyCode::S,
        KeyCode::D,
        KeyCode::Space,
        KeyCode::ShiftLeft,
        KeyCode::ControlLeft,
    ])
}

#[allow(clippy::type_complexity)]
fn handle_keyboard(
    player: Res<LocalPlayer>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, Or<(With<LocalPlayerComponent>, With<LocalPlayerHead>)>>,
) {
    // TODO: Add keybindings

    let Ok(transform) = query.get_mut(player.head) else {
        return;
    };

    let mut movement = Vec3::ZERO;
    if keyboard.pressed(KeyCode::W) {
        movement -= Vec3::Z;
    }
    if keyboard.pressed(KeyCode::S) {
        movement += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::A) {
        movement -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::D) {
        movement += Vec3::X;
    }

    movement = transform.rotation.mul_vec3(movement.normalize_or_zero());
    movement.y = 0.0;

    if keyboard.pressed(KeyCode::Space) {
        movement += Vec3::Y;
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        movement -= Vec3::Y;
    }

    if keyboard.pressed(KeyCode::ControlLeft) {
        movement *= 4.0;
    }

    if let Ok(mut transform) = query.get_mut(player.player) {
        transform.translation += movement * 0.125;
    }
}

fn any_mouse_events(events: EventReader<MouseMotion>) -> bool { !events.is_empty() }

fn handle_mouse(
    player: Res<LocalPlayer>,
    mut query: Query<&mut Transform, With<LocalPlayerHead>>,
    mut mouse: EventReader<MouseMotion>,
) {
    // TODO: Add keybindings

    let Ok(mut transform) = query.get_mut(player.head) else {
        return;
    };

    let delta = mouse.iter().fold(Vec2::ZERO, |acc, e| acc + e.delta);

    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
    yaw -= delta.x * 0.001;
    pitch -= delta.y * 0.001;
    pitch = pitch.clamp(-1.54, 1.54);

    transform.rotation = Quat::from_rotation_y(yaw) * Quat::from_rotation_x(pitch);
}
