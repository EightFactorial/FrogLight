use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::Settings;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CameraSettings {
    #[serde(default = "CameraSettings::default_fov")]
    pub fov: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            fov: Self::default_fov(),
        }
    }
}

impl CameraSettings {
    fn default_fov() -> f32 { 70.0 }

    /// Update the [Camera's](Camera) [`Projection`] when the [`Settings`] change.
    pub(super) fn update_camera(
        mut query: Query<&mut Projection, With<Camera3d>>,
        settings: Res<Settings>,
    ) {
        query.iter_mut().for_each(|mut proj| {
            if let Projection::Perspective(proj) = &mut *proj {
                if settings.camera.fov != proj.fov {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    debug!("Updating camera fov to {:?}", settings.camera.fov);

                    proj.fov = settings.camera.fov;
                }
            }
        });
    }
}

impl From<f32> for CameraSettings {
    fn from(fov: f32) -> Self { Self { fov } }
}

impl From<CameraSettings> for f32 {
    fn from(settings: CameraSettings) -> Self { settings.fov }
}
