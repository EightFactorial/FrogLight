//! Camera bundles for `FrogLight`.

use bevy::{prelude::*, render::camera::CameraOutputMode};

#[doc(hidden)]
pub(super) fn finish(app: &mut App) {
    app.register_type::<FrogLightCamera>();

    // If no cameras are present, add a 2D and 3D cameras.
    if app.world.query::<&Camera>().iter(&app.world).count() == 0 {
        debug!("No Cameras found, adding default cameras");
        app.world.spawn(FrogLightCamera2dBundle::new().clear());
        app.world.spawn(FrogLightCamera3dBundle::new().underneath());
    } else {
        debug!("Cameras found, skipping default camera creation");
    }
}

/// A marker [`Component`] for a `FrogLight` camera.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct FrogLightCamera;

/// A [`Bundle`] for a [`Camera2d`] with `FrogLight` settings.
#[derive(Bundle)]
pub struct FrogLightCamera2dBundle {
    name: Name,
    camera: Camera2dBundle,
    marker: FrogLightCamera,
    default: IsDefaultUiCamera,
}

impl FrogLightCamera2dBundle {
    /// The default `order` for the bundled [`Camera`].
    const DEFAULT_CAMERA_ORDER: isize = 16;

    /// Creates a new [`FrogLightCamera2dBundle`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Sets the [`Camera`]'s [`ClearColorConfig`] to `None`.
    #[must_use]
    pub fn clear(mut self) -> Self {
        self.camera.camera.clear_color = ClearColorConfig::None;
        self
    }
}

impl Default for FrogLightCamera2dBundle {
    fn default() -> Self {
        Self {
            name: Name::new("UiCamera"),
            camera: Camera2dBundle {
                camera: Camera { order: Self::DEFAULT_CAMERA_ORDER, ..Default::default() },
                ..Default::default()
            },
            marker: FrogLightCamera,
            default: IsDefaultUiCamera,
        }
    }
}

/// A [`Bundle`] for a [`Camera3d`] with `FrogLight` settings.
#[derive(Bundle)]
pub struct FrogLightCamera3dBundle {
    name: Name,
    camera: Camera3dBundle,
    marker: FrogLightCamera,
}

impl FrogLightCamera3dBundle {
    /// The default `order` for the bundled [`Camera`].
    const DEFAULT_CAMERA_ORDER: isize = -16;

    /// Creates a new [`FrogLightCamera3dBundle`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Enables optimizations for rendering underneath other cameras.
    ///
    /// See [`CameraOutputMode`] for more information.
    #[must_use]
    pub fn underneath(mut self) -> Self {
        self.camera.camera.output_mode = CameraOutputMode::Skip;
        self
    }
}

impl Default for FrogLightCamera3dBundle {
    fn default() -> Self {
        Self {
            name: Name::new("WorldCamera"),
            camera: Camera3dBundle {
                camera: Camera { order: Self::DEFAULT_CAMERA_ORDER, ..Default::default() },
                ..Default::default()
            },
            marker: FrogLightCamera,
        }
    }
}
