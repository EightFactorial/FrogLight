use serde::{Deserialize, Serialize};

use super::default_f32;

/// Settings for the game.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    #[serde(default)]
    pub camera: CameraSettings,
}

/// Settings for the camera.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraSettings {
    #[serde(default = "default_f32::<70>")]
    pub fov: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            fov: default_f32::<70>(),
        }
    }
}
