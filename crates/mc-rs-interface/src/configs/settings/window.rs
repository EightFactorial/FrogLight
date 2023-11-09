use bevy::window::{PresentMode, Window, WindowMode, WindowResolution};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowSettings {
    #[serde(default)]
    pub vsync: PresentMode,
    #[serde(default)]
    pub window: WindowMode,
    #[serde(default)]
    pub resolution: WindowResolution,
}

impl WindowSettings {
    pub fn into_window(self, title: String) -> Window {
        Window {
            title,
            present_mode: self.vsync,
            mode: self.window,
            resolution: self.resolution,
            ..Default::default()
        }
    }
}
