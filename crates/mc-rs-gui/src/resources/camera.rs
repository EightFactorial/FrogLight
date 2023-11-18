use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

/// A helper struct that contains the default camera settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultCamera;

impl DefaultCamera {
    /// Get the default [Camera2dBundle]
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

    /// Get the default [Camera3dBundle]
    pub fn default_camera3d() -> Camera3dBundle {
        Camera3dBundle {
            camera: Camera {
                order: isize::default(),
                ..Default::default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Get the default [Camera3dBundle] with a custom field of view
    pub fn camera3d_fov<const N: usize>() -> Camera3dBundle {
        let mut camera = Self::default_camera3d();
        camera.projection = Projection::Perspective(PerspectiveProjection {
            fov: N as f32,
            ..Default::default()
        });

        camera
    }
}
