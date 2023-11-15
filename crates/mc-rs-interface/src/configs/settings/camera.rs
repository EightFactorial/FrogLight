use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::Settings;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSettings {
    pub fov: f32,
}

impl Default for CameraSettings {
    fn default() -> Self { Self { fov: 70.0 } }
}

impl CameraSettings {
    /// Update the camera settings.
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
