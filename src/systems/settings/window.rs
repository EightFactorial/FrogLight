use bevy::window::{PresentMode, WindowMode, WindowResolution};
use serde::{Deserialize, Serialize};

/// Settings for the application window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSettings {
    #[serde(default)]
    pub resolution: WindowResolution,
    #[serde(default)]
    pub window_mode: WindowMode,
    #[serde(default = "default_present_mode")]
    pub present_mode: PresentMode,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            resolution: Default::default(),
            window_mode: Default::default(),
            present_mode: default_present_mode(),
        }
    }
}

/// Get the default [PresentMode].
#[inline]
const fn default_present_mode() -> PresentMode { PresentMode::AutoVsync }
