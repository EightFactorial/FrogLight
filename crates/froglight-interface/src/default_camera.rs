//! Default camera bundles.

use bevy::prelude::*;

/// The default [`Camera2dBundle`] bundle group.
///
/// Includes:
/// - [`UiCameraConfig`] with `show_ui` set to `true`
/// - [`Camera2dBundle`] with order set to `1` and color set to [`Color::NONE`]
#[must_use]
pub fn default_camera2d_bundle() -> impl Bundle {
    (
        IsDefaultUiCamera,
        Camera2dBundle {
            camera: Camera {
                order: 1isize,
                is_active: true,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
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
    Camera3dBundle {
        camera: Camera {
            order: 0isize,
            is_active: true,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..Default::default()
        },
        projection: Projection::Perspective(PerspectiveProjection {
            fov: 70f32.to_radians(),
            ..Default::default()
        }),
        ..Default::default()
    }
}
