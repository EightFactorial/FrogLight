use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

/// A helper struct that contains the default camera settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultCamera;

impl DefaultCamera {
    /// Get the default [`Camera2dBundle`]
    #[must_use]
    pub fn default_camera2d() -> Camera2dBundle {
        Camera2dBundle {
            camera: Camera {
                order: isize::MAX - 32,
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..Default::default()
        }
    }

    /// A [`bevy`] system that enables all [`Camera2d`]s
    pub fn enable_camera2d(mut query: Query<&mut Camera, With<Camera2d>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Enabling Camera2d");

        query.iter_mut().for_each(|mut camera| {
            camera.is_active = true;
        });
    }

    /// A [`bevy`] system that disables all [`Camera2d`]s
    pub fn disable_camera2d(mut query: Query<&mut Camera, With<Camera2d>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Disabling Camera2d");

        query.iter_mut().for_each(|mut camera| {
            camera.is_active = false;
        });
    }

    /// Get the default [`Camera3dBundle`]
    #[must_use]
    pub fn default_camera3d() -> Camera3dBundle {
        Camera3dBundle {
            camera: Camera {
                order: isize::default(),
                ..Default::default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.53, 0.81, 0.92)),
                ..Default::default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 70f32.to_radians(),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    /// A [bevy] system that enables all [Camera3d]s
    pub fn enable_camera3d(mut query: Query<&mut Camera, With<Camera3d>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Enabling Camera3d");

        query.iter_mut().for_each(|mut camera| {
            camera.is_active = true;
        });
    }

    /// A [bevy] system that disables all [Camera3d]s
    pub fn disable_camera3d(mut query: Query<&mut Camera, With<Camera3d>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Disabling Camera3d");

        query.iter_mut().for_each(|mut camera| {
            camera.is_active = false;
        });
    }
}
