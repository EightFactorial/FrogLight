use bevy::window::WindowResolution;
use serde::{Deserialize, Serialize};

/// Settings for the application window.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WindowSettings {
    pub resolution: WindowResolution,
}
