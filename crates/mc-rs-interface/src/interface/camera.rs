use bevy::{
    core_pipeline::clear_color::ClearColorConfig, ecs::schedule::SystemConfigs, prelude::*,
};

use crate::configs::settings::Settings;

/// A container for all the systems related to the camera.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DefaultCamera;

impl DefaultCamera {
    /// Create [SystemConfigs] to create a default 2d camera.
    pub fn create_camera2d() -> SystemConfigs {
        DefaultCamera::default_camera2d.run_if(not(any_with_component::<Camera2d>()))
    }

    /// A system that creates the default [Camera2dBundle].
    ///
    /// You should likely use [`DefaultCamera::create_camera2d`](DefaultCamera) instead.
    pub fn default_camera2d(mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Spawning Camera2d");

        commands.spawn(Camera2dBundle {
            camera: Camera {
                // Put the camera2d in front
                order: isize::MAX - 8,
                is_active: true,
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..Default::default()
        });
    }

    /// A system that enables all [Camera2d]s.
    pub fn enable_camera2d(mut query: Query<&mut Camera, With<Camera2d>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Enabling all Camera2d");

        query.iter_mut().for_each(|mut camera| {
            camera.is_active = true;
        });
    }

    /// A system that disables all [Camera2d]s.
    pub fn disable_camera2d(mut query: Query<&mut Camera, With<Camera2d>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Disabling all Camera2d");

        query.iter_mut().for_each(|mut camera| {
            camera.is_active = false;
        });
    }

    /// A system that destroys all [Camera2d]s.
    pub fn destroy_camera2d(query: Query<Entity, With<Camera2d>>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Despawning all Camera2d");

        query.iter().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });
    }

    /// Create [SystemConfigs] to create a default 3d camera.
    pub fn create_camera3d() -> SystemConfigs {
        DefaultCamera::default_camera3d.run_if(not(any_with_component::<Camera3d>()))
    }

    /// A system that creates the default [Camera3dBundle].
    ///
    /// You should likely use [`DefaultCamera::create_camera3d`](DefaultCamera) instead.
    pub fn default_camera3d(settings: Res<Settings>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Spawning Camera3d with FoV: {}", settings.camera.fov);

        commands.spawn(Camera3dBundle {
            camera: Camera {
                // Put the camera3d in the middle
                order: 0isize,
                is_active: true,
                ..Default::default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: settings.camera.fov.to_radians(),
                ..Default::default()
            }),
            ..Default::default()
        });
    }

    /// A system that creates a [Camera3dBundle] with a custom FoV.
    pub fn custom_fov_camera3d<const N: usize>(mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Spawning Camera3d with FoV: {}", N);

        commands.spawn(Camera3dBundle {
            camera: Camera {
                // Put the camera3d in the middle
                order: 0isize,
                is_active: true,
                ..Default::default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: (N as f32).to_radians(),
                ..Default::default()
            }),
            ..Default::default()
        });
    }

    /// A system that enables all [Camera3d]s.
    pub fn enable_camera3d(mut query: Query<&mut Camera, With<Camera3d>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Enabling all Camera3d");

        query.iter_mut().for_each(|mut camera| {
            camera.is_active = true;
        });
    }

    /// A system that disables all [Camera3d]s.
    pub fn disable_camera3d(mut query: Query<&mut Camera, With<Camera3d>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Disabling all Camera3d");

        query.iter_mut().for_each(|mut camera| {
            camera.is_active = false;
        });
    }

    /// A system that destroys all [Camera3d]s.
    pub fn destroy_camera3d(query: Query<Entity, With<Camera3d>>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Despawning all Camera3d");

        query.iter().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });
    }
}
