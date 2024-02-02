//! Default camera bundles.

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

/// The default [`Camera2dBundle`] bundle group.
///
/// Includes:
/// - [`UiCameraConfig`] with `show_ui` set to `true`
/// - [`Camera2dBundle`] with order set to `1` and color set to [`Color::NONE`]
#[must_use]
pub fn default_camera2d_bundle() -> impl Bundle {
    (
        UiCameraConfig { show_ui: true },
        Camera2dBundle {
            camera: Camera { order: 1isize, is_active: true, ..Default::default() },
            camera_2d: Camera2d { clear_color: ClearColorConfig::Custom(Color::NONE) },
            ..Default::default()
        },
    )
}

/// The default [`Camera3dBundle`] bundle group.
///
/// Includes:
/// - [`UiCameraConfig`] with `show_ui` set to `false`
/// - [`Camera3dBundle`] with order set to `0` and color set to [`Color::BLACK`]
#[must_use]
pub fn default_camera3d_bundle() -> impl Bundle {
    (
        UiCameraConfig { show_ui: false },
        Camera3dBundle {
            camera: Camera { order: 0isize, is_active: true, ..Default::default() },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 70f32.to_radians(),
                ..Default::default()
            }),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
